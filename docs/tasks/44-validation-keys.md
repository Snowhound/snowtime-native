# 081.44: Validation failures answer a key

Status: done (2026-10-22): the bar passes at `40d9d35`

Follow snowtime task 095 (PR #8, merged as `40d9d35`): input that fails its schema answers
400 with `{ key, params, message }`, the message in English whatever the request's locale.
The pin moves from `981832f` to `40d9d35`.

## Changes

- `errors.rs`: `Error::Invalid` carries an `Invalid` with the key, at most one parameter
  (`max`, `hours`, `days`, or `key`), and the English message. A body that isn't JSON is
  `Error::NotJson`, which still answers a plain message, as the app does.
- `schemas.rs`: `refuse_input` and `refuse_input_with` answer a keyed message, rendered from
  a table of the app's English texts; a test compares the table with the pinned app's
  `messages/en.json`. `invalid` stays for valibot's own checks, which answer
  `validation_invalid` with valibot's message.
- `wire.rs`: `invalid_failure` serializes the struct, so the body keeps the app's key order
  (`key`, `params`, `message`) whether or not a crate in the build turns on serde_json's
  `preserve_order`.
- The domain schemas' 25 refusals name their keys.

The other app commits since `981832f` change files the port builds rather than ports: the
public `robots.txt`, `favicon.ico`, and `apple-touch-icon.png`, which the adapter copies,
and client code in the bundle (the weather shaders, the client's error handling).

## Acceptance criteria

- [x] The bar passes at the new pin: the app's conformance suite, including task 095's
      Estonian-session test, and the byte comparison of every call.
- [x] README names the new pin, with 67 conformance tests and 572 assertions.
