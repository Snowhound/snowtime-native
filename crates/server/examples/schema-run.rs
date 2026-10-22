//! Reads `[name, value]` JSON lines and prints the generated input schema's verdict on each,
//! for the porting kit's differential test (schema-gen.json).
use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    for line in std::io::stdin().lock().lines() {
        let (name, value): (String, serde_json::Value) =
            serde_json::from_str(&line.unwrap()).expect("a [name, value] line");
        let verdict = snowtime_server::inputs::run(&name, value)
            .unwrap_or_else(|| panic!("no input schema {name}"));
        writeln!(out, "{verdict}").unwrap();
    }
}
