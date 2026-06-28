# Stage Default Directories (v0.4.4+)

> User-editable: edit this file to customize stage directories.
> `ek init` reads this file → creates default stage directories.
> If file is missing or empty → fallback to v0.4.1 defaults (backward compatible).

## Default Directories

| name | purpose | files |
|------|---------|-------|
| `tasks` | Active task records (P1-P5) | summary/requirements/design/plan/changes/tests/delivery/review |
| `drafts` | Knowledge drafts (promote → KB) | (user writes freely) |
| `.archive` | Completed/archived tasks | (auto) |

## Default Task Files (under `tasks/<task-id>/`)

- `summary.md` — task overview
- `requirements.md` — EARS format requirements
- `design.md` — design rationale
- `plan.md` — implementation plan
- `changes.md` — append-only change log
- `tests.md` — test records
- `delivery.md` — delivery summary
- `review.md` — AI pre-fills, human ticks
