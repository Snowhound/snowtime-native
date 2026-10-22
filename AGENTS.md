# Agent rules

## Project context

- What this is, the layout, and the pin: `README.md`
- Current design: `docs/architecture/`; task records, with what was tried and measured:
  `docs/tasks/` (see `docs/tasks/README.md`)
- The app this ports is the `app/` submodule at the pinned commit. Never edit it here;
  apply `deploy/load/app.patch` only for a load or gate run, then revert it
  (`deploy/load/README.md`).
- The porting kit (github.com/kaitk/porting-kit) holds the recipes this port proved.

Docs and commits name no people: date a decision, and write "the maintainer" where an
approval is needed.

## Commits

- One lean sentence in imperative mood.
- No body, no `Co-Authored-By` or any other trailers.

## Pull requests

- Plain prose about the change: what, why, and one line on the checks.
- No "Generated with Claude Code" line or any other attribution.
- Run a new pull request's text through `app/.claude/skills/google-style` when you first
  write it.
