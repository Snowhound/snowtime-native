//! The port's side of the porting kit's SQL capture (tools/sql-capture): with the
//! `sql-capture` feature and `SQL_CAPTURE` naming a file, every statement SQLite runs is
//! written there as a JSON line, tagged with the request that ran it, and each request
//! adds a line when it ends. The kit compares it with the app's capture per route
//! (harness/sql-capture/compare.sh).
//!
//! A request's id lives in a task-local while its handler runs, and a lane's job carries it
//! to the worker thread that runs its statements (lane.rs).
use std::cell::Cell;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use axum::{Router, extract::Request, middleware::Next, response::Response};
use rusqlite::trace::StmtRef;
use serde_json::json;

tokio::task_local! {
    static REQUEST: u64;
}

thread_local! {
    // The request whose lane job runs on this thread.
    static CURRENT: Cell<Option<u64>> = const { Cell::new(None) };
}

static NEXT: AtomicU64 = AtomicU64::new(0);

fn file() -> Option<&'static Mutex<File>> {
    static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    FILE.get_or_init(|| {
        let path = std::env::var_os("SQL_CAPTURE")?;
        let file = File::options().create(true).append(true).open(path);
        Some(Mutex::new(file.expect("SQL_CAPTURE opens")))
    })
    .as_ref()
}

fn write(line: serde_json::Value) {
    if let Some(file) = file() {
        let mut file = file.lock().unwrap_or_else(|e| e.into_inner());
        let _ = writeln!(file, "{line}");
    }
}

fn id(request: u64) -> String {
    format!("r{}.{request}", std::process::id())
}

/// The request running on this thread or task, to hand to a lane's job.
pub(crate) fn current() -> Option<u64> {
    CURRENT
        .with(Cell::get)
        .or_else(|| REQUEST.try_with(|r| *r).ok())
}

/// Runs a lane's job as part of `request`.
pub(crate) fn with<T>(request: Option<u64>, job: impl FnOnce() -> T) -> T {
    let before = CURRENT.with(|c| c.replace(request));
    let result = job();
    CURRENT.with(|c| c.set(before));
    result
}

/// A statement SQLite finished, from the trace hook (timing.rs). Parameters aren't counted.
pub(crate) fn statement(stmt: &StmtRef<'_>, took: Duration) {
    if file().is_none() {
        return;
    }
    write(json!({
        "kind": "statement",
        "request": current().map(id),
        "statements": [{ "sql": stmt.sql(), "params": 0 }],
        "ms": took.as_secs_f64() * 1000.0,
    }));
}

async fn capture(request: Request, next: Next) -> Response {
    let own = NEXT.fetch_add(1, Ordering::Relaxed) + 1;
    let parent = REQUEST.try_with(|r| *r).ok();
    let method = request.method().to_string();
    let url = request
        .uri()
        .path_and_query()
        .map_or_else(|| request.uri().path().to_owned(), |p| p.to_string());
    let started = Instant::now();
    let response = REQUEST.scope(own, next.run(request)).await;
    write(json!({
        "kind": "request",
        "request": id(own),
        "parent": parent.map(id),
        "method": method,
        "url": url,
        "status": response.status().as_u16(),
        "ms": started.elapsed().as_secs_f64() * 1000.0,
    }));
    response
}

/// `router` with each request captured.
pub fn layer<S: Clone + Send + Sync + 'static>(router: Router<S>) -> Router<S> {
    router.layer(axum::middleware::from_fn(capture))
}
