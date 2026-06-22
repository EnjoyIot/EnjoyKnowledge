<!-- OMC:START -->
<!-- OMC:VERSION:4.14.4 -->

# oh-my-Codex - Intelligent Multi-Agent Orchestration

You are running with oh-my-Codex (OMC), a multi-agent orchestration layer for Codex.
Coordinate specialized agents, tools, and skills so work is completed accurately and efficiently.

<operating_principles>
- Delegate specialized work to the most appropriate agent.
- Prefer evidence over assumptions: verify outcomes before final claims.
- Choose the lightest-weight path that preserves quality.
- Consult official docs before implementing with SDKs/frameworks/APIs.
</operating_principles>

<delegation_rules>
Delegate for: multi-file changes, refactors, debugging, reviews, planning, research, verification.
Work directly for: trivial ops, small clarifications, single commands.
Route code to `executor` (use `model=opus` for complex work). Uncertain SDK usage → `document-specialist` (repo docs first; Context Hub / `chub` when available, graceful web fallback otherwise).
</delegation_rules>

<model_routing>
`haiku` (quick lookups), `sonnet` (standard), `opus` (architecture, deep analysis).
Direct writes OK for: `~/.Codex/**`, `.omc/**`, `.Codex/**`, `AGENTS.md`, `AGENTS.md`.
</model_routing>

<skills>
Invoke via `/oh-my-Codex:<name>`. Trigger patterns auto-detect keywords.
Tier-0 workflows include `autopilot`, `ultrawork`, `ralph`, `team`, and `ralplan`.
Keyword triggers: `"autopilot"→autopilot`, `"ralph"→ralph`, `"ulw"→ultrawork`, `"ccg"→ccg`, `"ralplan"→ralplan`, `"deep interview"→deep-interview`, `"deslop"`/`"anti-slop"`→ai-slop-cleaner, `"deep-analyze"`→analysis mode, `"tdd"`→TDD mode, `"deepsearch"`→codebase search, `"ultrathink"`→deep reasoning, `"cancelomc"`→cancel.
Team orchestration is explicit via `/team`.
Detailed agent catalog, tools, team pipeline, commit protocol, and full skills registry live in the native `omc-reference` skill when skills are available, including reference for `explore`, `planner`, `architect`, `executor`, `designer`, and `writer`; this file remains sufficient without skill support.
</skills>

<verification>
Verify before claiming completion. Size appropriately: small→haiku, standard→sonnet, large/security→opus.
If verification fails, keep iterating.
</verification>

<execution_protocols>
Broad requests: explore first, then plan. 2+ independent tasks in parallel. `run_in_background` for builds/tests.
Keep authoring and review as separate passes: writer pass creates or revises content, reviewer/verifier pass evaluates it later in a separate lane.
Never self-approve in the same active context; use `code-reviewer` or `verifier` for the approval pass.
Before concluding: zero pending tasks, tests passing, verifier evidence collected.
</execution_protocols>

<hooks_and_context>
Hooks inject `<system-reminder>` tags. Key patterns: `hook success: Success` (proceed), `[MAGIC KEYWORD: ...]` (invoke skill), `The boulder never stops` (ralph/ultrawork active).
Persistence: `<remember>` (7 days), `<remember priority>` (permanent).
Kill switches: `DISABLE_OMC`, `OMC_SKIP_HOOKS` (comma-separated).
</hooks_and_context>

<cancellation>
`/oh-my-Codex:cancel` ends execution modes. Cancel when done+verified or blocked. Don't cancel if work incomplete.
</cancellation>

<worktree_paths>
State: `.omc/state/`, `.omc/state/sessions/{sessionId}/`, `.omc/notepad.md`, `.omc/project-memory.json`, `.omc/plans/`, `.omc/research/`, `.omc/logs/`
</worktree_paths>

## Setup

Say "setup omc" or run `/oh-my-Codex:omc-setup`.

<!-- OMC:END -->

---

# enjoyknowledge Project

## What This Is

enjoyknowledge is a Rust CLI that provides **shared context for AI coding tools** — it organizes project knowledge (architecture, conventions, gotchas, business rules) as OKF-compatible Markdown under `.enjoyknowledge/`, so any AI tool can discover, read, and contribute to the same knowledge base.

**Stack**: Rust 2021 edition, clap (derive), anyhow+thiserror, serde_yaml, regex, walkdir.

## Single Verification Command

```bash
just check
```

This runs: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features`.
**Never claim a task is done until `just check` passes with zero errors.**

## Design Contract

All CLI behavior is specified in `docs/INTERFACE-SPEC.md`. This is the **binding contract** — output format, error codes, argument names. When implementing a command, verify against this doc, not against assumptions.

Key spec sections:
- §2: `.enjoyknowledge/` directory structure
- §3: OKF-compatible Markdown frontmatter
- §4: CLI commands (`ls`, `tree`, `grep`, `cat`, `add`, `init`, `doctor`, `fix`)
- §7: Error codes (0=success, 1=arg error, 2=not found, 3=format, 4=file unreadable/unwritable)

## Module Map

```
src/
├── main.rs           → clap CLI dispatch
├── config.rs         → CLI configuration helpers, if needed
├── cli/              → command handlers (thin — delegate to modules)
│   ├── init.rs, ls.rs, tree.rs, grep.rs, cat.rs, add.rs, doctor.rs
├── knowledge/        → KnowledgeSource trait + FilesystemSource + Index
├── format/           → YAML frontmatter parsing + Markdown section extraction
├── init/             → skeleton generation, templates, AI tool files
├── doctor/           → 8 health checks, dedup algorithm, budget/archive
└── add/              → append/create logic, type inference, AGENTS.md summary sync
```

## Development Rules

1. **Contract-first**: Before implementing a CLI command, read the corresponding spec in INTERFACE-SPEC.md
2. **Test every command**: Use trycmd (`.trycmd/`) for CLI snapshot tests — add a `.toml` or `.stdin` test case for new behavior
3. **Add error types**: New failure modes → new variant in a `thiserror` enum, not `anyhow!`
4. **grep is structure-aware**: `grep` matches Markdown body sections and filters with frontmatter `type` / `tags`
5. **add is append-first**: Existing files receive new content at the end; creating a missing file must generate valid frontmatter
6. **Every `.md` knowledge file must have frontmatter**: `type` is required; `description`, `tags`, and `timestamp` are recommended

## Feature Development Process (MANDATORY)

**This is the only valid way to add or change features.** Follow these steps in order. Do not skip.

### Step 1: Spec First
Read `docs/INTERFACE-SPEC.md` to understand the current contract.
If the feature changes CLI behavior (new command, new flag, new output format):
- Update `docs/INTERFACE-SPEC.md` with the new contract
- Update `docs/DESIGN-V3.md` if the implementation architecture changes
- Update `docs/CHANGELOG.md` under the current version section

### Step 2: Test First
Add trycmd test cases in `tests/cmd/` that define the EXPECTED behavior:
```
tests/cmd/<feature-name>.trycmd
```
Each test case is a `.trycmd` file with the exact `$ enjoyknowledge ...` command and expected stdout.
If the feature needs new test data, add it to `tests/fixtures/minimal-project/.enjoyknowledge/`.
Then un-ignore the trycmd test in `tests/cli.rs` by removing `#[ignore]`.

### Step 3: Implement
Write the code. Follow the module map above:
- New CLI flags → `src/main.rs` clap derive
- New command → `src/cli/<command>.rs` + handler in corresponding `src/<module>/`
- New knowledge source → implement `KnowledgeSource` trait in `src/knowledge/`
- New error cases → add variant to the relevant `thiserror` enum

### Step 4: Verify
```bash
just check
```
Must pass with **zero errors, zero warnings**. If trycmd tests fail, compare actual output against `tests/cmd/*.trycmd` — either fix the code (if output is wrong) or update the test (if the spec changed).

### Step 5: Self-Review
Before claiming done, answer:
- [ ] Did I update INTERFACE-SPEC.md if the contract changed?
- [ ] Did I add trycmd tests for the new behavior?
- [ ] Did `just check` pass with zero errors?
- [ ] Did I update CHANGELOG.md?

## Bug Fix Process

1. **Reproduce first**: Add a trycmd test that demonstrates the bug (expected ≠ actual)
2. Fix the code
3. `just check` — the new test should now pass
4. If the bug was a contract violation, note it in CHANGELOG.md

## When Stuck

- Design docs: `docs/DESIGN-V3.md` (how it works), `docs/PRODUCT-DESIGN.md` (user flows), `docs/GLOSSARY.md` (terms)
- Interface contract: `docs/INTERFACE-SPEC.md` (what correct output looks like)
- Read the trait definition: `src/knowledge/source.rs` (the central abstraction)
- Run `just check` — the compiler and tests are the ultimate authority
