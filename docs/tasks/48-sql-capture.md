# 081.48: SQL per route

Status: in-progress (2026-10-22): captured and compared; three port findings open

Record the SQL the app runs for each route as the porting aid, then record the port the
same way and compare per route: calls, statements, repeated statements, transactions.
This port proves the kit's SQL capture (kit task 004, `tools/sql-capture`) before it
enters the kit.

## Changes

- `harness/sql-capture/app.patch` connects the kit's libsql adapter for a capture run
  only: around the client in `src/db/index.ts`, around each request in
  `src/server-entry.ts` and each page loader's API call made in process, and passes
  `SQL_CAPTURE` to the server in `perf/lib/app.ts`. `capture.sh` applies it, copies the
  adapter in, runs the app's conformance suite, reverts both, and writes `sql/`.
- The `sql-capture` feature (`crates/server/src/sql_capture.rs`) writes the same format
  from the port: SQLite's trace hook writes each statement, a layer on the API router and
  on the pages router tags requests, and a lane's job carries its request to the worker
  thread. CI lints it beside `bench`.
- `compare.sh` builds the port with the feature, runs `harness/conformance.ts` with
  `SQL_CAPTURE`, and compares per route. `route-parity.json`'s `sqlCapture.accepted`
  lists the one difference made on purpose: one more statement per request, because the
  session check reads the session row on every request (task 081.03).

## Results

Measured 2026-10-22 on macOS, pin `40d9d35`:

- **App:** 67 of 67 conformance tests passed. The capture holds 2,508 lines and 69
  routes, with no calls outside a request.
- **Port:** the OAuth conformance file fails locally on its Latin-1 `Location` check,
  also on `main` without these changes (CI passed at `4a51936`). Its three routes are
  missing from the port's capture, and the request counts of 8 routes differ.
- **Comparison:** 69 routes, 15 with notes. 8 of the notes are request counts only.

Findings in the port, open:

1. `GET /api/v1/session` reads the session row twice per request (in 6 of 8 requests).
2. `PATCH /api/v1/settings` updates and then selects the settings, 3 statements to the
   app's 1. The app's `update … returning` does both at once.
3. `GET /api/auth/passkey/generate-register-options` reads the user's email after the
   session check has already joined it.

`POST /api/v1/invitations/:id/accept` runs 8 statements in 1 transaction, where the app
runs 13 in 2.

Findings in the app, for snowtime: Better Auth's organization endpoints and invitations
read the session or the user two or three times per request, and the report export
reads `user_settings` twice. These are fixed duplicate reads, not loops that grow with
the data.

## Acceptance criteria

- [x] The app's SQL per route in `sql/`, from its conformance suite at the pin.
- [x] The port writes the same format behind a feature, and the comparison runs per route.
- [ ] The three port findings fixed, or accepted with a reason.
- [ ] The OAuth conformance file passes locally, or the failure is recorded as a known
      local issue.
- [ ] `harness/kit.sh` names a pushed kit revision with the tool.
