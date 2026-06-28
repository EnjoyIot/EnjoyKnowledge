---
name: enjoyknowledge-stage
description: "Stage writing conventions for AI tools during coding tasks. Use when starting a new task, deciding what to write, or reviewing the project. Triggers on '做任务' / 'stage 写什么' / '改 stage AGENTS.md' / 'extend stage' / 'add stage dir' / 'task phase P1-P5'."
version: 1.0.0
metadata:
  hermes:
    tags: [stage, task, coding, ai-context]
    related_skills: [enjoyknowledge]
---

# enjoyknowledge Stage — Task Writing Spec (v0.4.4+)

## Overview

This file tells AI tools **how to use `.enjoyknowledge_stage/`** during coding tasks.
**User-editable**: edit this file directly to change stage conventions.
`ek init` will **never overwrite** this file (it's user-owned).

## Inputs (what user provides)

- Task ID (e.g., `2026-06-28-add-kind-registry`)
- Task description (1-3 sentences)

## Workflow (5 Phases x 3 Hard Gates)

### P1 Requirements
- Write to: `tasks/<task-id>/requirements.md`
- EARS format (Event -> Action -> Response -> State)
- Hard Gate 1: human approval

### P2 Design
- Write to: `tasks/<task-id>/design.md`
- Hard Gate 2: human approval

### P2b Plan
- Write to: `tasks/<task-id>/plan.md`

### P3 Coding
- Write to: `tasks/<task-id>/changes.md` (append-only)
- One line per file edit (old -> new summary)

### P4 Testing
- Write to: `tasks/<task-id>/tests.md`
- Before first test run + after each run

### P5 Delivery
- Write to: `tasks/<task-id>/delivery.md`
- Hard Gate 3: human approval
- After: `ek promote <draft> --to <kind>`

## Custom Directories (user-editable)

If user added custom directories in `_meta/stage-defaults.md`:
- `notes/<file>.md` — user notes
- `experiments/<file>.md` — experiment records
- ...

AI should use these directories according to user's stage AGENTS.md updates.

## Promote Workflow

1. AI writes draft to `.enjoyknowledge_stage/drafts/<id>.md`
2. Human reviews + runs `ek promote <draft> --to <kind>`
3. Draft gets `[PROMOTED]` marker

## Hard Gate Protocol

```
P1 req -[H1]-> P2 design -[H2]-> P3 coding -> P4 test -> P5 delivery -[H3]-> promote
```

---
*User-owned: edit this file to customize stage conventions. `ek init` will not overwrite.*
