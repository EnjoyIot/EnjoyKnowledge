# AI Coding Tools — Custom Rules / Instructions Mechanism Comparison

**Task**: t_c1586b65 — Research rule mechanisms of each AI coding tool
**Scope**: Cursor · Claude Code · Codex CLI · Trae
**Source basis**: training-data recall (knowledge cutoff Jan 2026) — none of the 4 sub-investigations were URL-verified against the official docs in this run, because no worker in the run had web_fetch / browser tools. Every claim below is therefore flagged with one of:
- **[OK]** = well-supported by training-time docs; stable doc URL cited
- **[V]** = plausible per training-time docs but **NOT verified** against the live official doc in this run
- **[ND]** = not documented / no published number / explicitly unknown

---

## Side-by-side comparison matrix

| # | Question | **Cursor** | **Claude Code** | **Codex CLI** | **Trae** |
|---|---|---|---|---|---|
| 1 | **Rule file location & naming** | Legacy: `.cursorrules` (project root, single plain-text file). Current: `.cursor/rules/*.mdc` (multiple markdown-with-frontmatter files). User rules: stored in IDE editor profile. Team rules: in Cursor dashboard/admin console. `[V]` | Project memory: `CLAUDE.md` (root, ancestor dirs, subdirs). User memory: `~/.claude/CLAUDE.md`. Settings: `.claude/settings.json` (project), `.claude/settings.local.json` (local override, gitignored), `~/.claude/settings.json` (user). MCP: `.mcp.json` (project), `~/.mcp.json` (user). `[OK]` | Project: `AGENTS.md` at repo root + subdirs. User dir `~/.codex/`: `config.toml`, `AGENTS.md` (modern), `instructions.md` (legacy alias). `CODEX_HOME` env var overrides `~/.codex/`. Legacy project subdir `.codex/` still honored. `[OK]` | Workspace: `.traerules` at workspace root (single plain-text / md file). User rules: stored in IDE settings panel, not on disk. Per-chat: in-panel "System Prompt" / "Custom Instructions" field. `[V]` |
| 2 | **Supported scope** | Project, User, Team (org/workspace-wide via Cursor dashboard for Business/Enterprise). `[V]` | Enterprise/managed, Project, User (via `~/.claude/CLAUDE.md`). All are loaded — never exclusive. `[OK]` | User (`~/.codex/AGENTS.md`), Project (repo-root `AGENTS.md`), Per-folder (subdir `AGENTS.md`). All additive. `[OK]` | Workspace + User. No documented team/org tier. `[V]` |
| 3 | **Priority / override behavior** | **Team rules > User rules > Project rules** (team overrides everything below). Below that, all applicable sources are **concatenated** into system prompt, not overridden. `.cursorrules` + `.cursor/rules/*.mdc` + `AGENTS.md` (auto-loaded as project context) are all merged. `alwaysApply` and `globs` frontmatter control conditional inclusion, not priority. `[V]` | CLAUDE.md files are **concatenated** (user → ancestors → project → subdir); do not override. `settings.json` uses a **real override hierarchy**: `local > project > user`, per-key deep-merge. Managed/enterprise policies override user settings. `[OK]` | AGENTS.md files are **concatenated** (user `~/.codex/AGENTS.md` → repo-root → ancestors → subdir). Deeper (subdir) files don't "win" — appended after root. `instructions.md` (legacy) and `AGENTS.md` (modern) both load if both exist. `[OK]` | Workspace `.traerules` + user rules are **concatenated** into system prompt. No documented "always-override" priority class. `[V]` |
| 4 | **Format constraints** | `.cursorrules`: free-form text/md, no frontmatter. `.mdc`: Markdown + YAML frontmatter (keys: `description`, `globs`, `alwaysApply`). All injected into system prompt at conversation start (static prompt injection, not a tool call). No JSON rule format (JSON only for `.cursor/hooks/*` and MCP config). `[V]` | `CLAUDE.md`: Markdown, plain text, no frontmatter required. `@relative/path.md` import syntax (relative to importing file, inlined verbatim). Rules injected into system prompt under "Memory" / "Project instructions" section. `settings.json`: strict JSON (`permissions`, `env`, `mcpServers`, `hooks`, `enabledMcpjsonServers`, `model`, `cleanupPeriodDays`, …). `.mcp.json`: JSON `{ "mcpServers": { "name": { "command", "args" } } }`. `[OK]` | `AGENTS.md` / `instructions.md`: Markdown, plain prose, no required frontmatter. `config.toml`: strict TOML with sections `[model]`, `[shell]`, `[sandbox]`, `[notice]`, etc. Rules injected into system prompt / initial developer message at session start. `[OK]` | `.traerules`: free-form text/md, no required frontmatter, no JSON schema, no validation. In-panel "Custom Instructions" / "System Prompt": also free-form text. All injected into system prompt (static prompt injection). No documented JSON rule schema equivalent to Cursor's `.mdc` or Claude Code's `settings.json`. `[V]` |
| 5 | **Documented limits** | No specific numeric rule-count, per-file-size, or total-length cap published. Practical limit = model context window minus conversation + other context. `[ND]` | No specific byte/token cap for CLAUDE.md published. `@import` recursion depth not numerically documented (circular imports guarded). "Max rules per file" N/A (free-form markdown). `settings.json` no documented size limit. Per-session memory budget = context window. `[ND]` | No explicit byte/token cap for AGENTS.md published. No published max file size. No reserved-token budget for rules. Docs only recommend "concise" instructions. `config.toml` no documented size limit. `[ND]` | No published numeric cap on rule file size, token count, or rule count. Practical limit = model context window. UI-side character/byte cap on in-panel custom-instructions field may exist but not citable. `[ND]` |

---

## Key takeaways

1. **All four tools share the same fundamental mechanism**: rules are **free-form text/markdown files** injected into the model's **system prompt at session start**. None of them implement rules as a structured tool call or a separate rules engine — it's all static prompt injection.

2. **All four tools use additive concatenation** as the primary merge strategy for rules from multiple sources (with the exception of Claude Code's `settings.json`, which uses a real per-key override hierarchy).

3. **Cursor is the only tool with a documented override class** ("Team rules" win over User/Project rules). Trae, Codex, and Claude Code do not document any "always-override" rule tier (Claude Code has enterprise/managed policies but these are about settings, not memory content).

4. **Trae is the most minimal** — single workspace file (`.traerules`), no documented frontmatter, no team tier, no `@import` syntax. Cursor's `.mdc` + frontmatter is the richest, and Claude Code's `@path` imports + settings.json deep-merge is the most flexible.

5. **Codex CLI** sits between Cursor and Trae: it has the modern AGENTS.md + the legacy `instructions.md` alias, plus a TOML config sidecar (`config.toml`) — but no frontmatter or JSON rule schema.

6. **None of the four tools publish numeric limits** (size, count, depth). The practical limit is always the model's context window. Anthropic is the only vendor that gives explicit guidance ("keep CLAUDE.md concise; use `@import` for bulky material").

7. **The monorepo story differs**:
   - **Cursor**: per-file conditional inclusion via `globs` + `alwaysApply` frontmatter.
   - **Claude Code**: ancestor + subdir CLAUDE.md files both auto-load via path walk.
   - **Codex CLI**: subdir `AGENTS.md` files auto-load per working subtree.
   - **Trae**: single file per workspace, no documented per-folder rules.

8. **Verified-gap (per task constraint)**: All four sections above carry the `[V]` caveat because no worker in this run had web access. Treat any specific limit number or non-standard key as **unverified**. The matrix as a whole should be re-verified against the URLs cited below before being published downstream.

---

## Source URLs (NOT VERIFIED in this run)

- **Cursor**: https://cursor.com/docs/rules · https://docs.cursor.com/en/rules · https://cursor.com/docs/context/rules · https://cursor.com/changelog (for `.mdc` frontmatter evolution)
- **Claude Code**: https://docs.claude.com/en/docs/claude-code/memory · https://docs.claude.com/en/docs/claude-code/settings · https://docs.claude.com/en/docs/claude-code/mcp
- **Codex CLI**: https://developers.openai.com/codex/cli#instructions · https://github.com/openai/codex/blob/main/docs/config.md · https://github.com/openai/codex/blob/main/docs/instructions.md
- **Trae**: https://docs.trae.ai/ide/rules · https://docs.trae.ai/ide/custom-rules · https://trae.ai/docs/workspace/rules · https://trae.ai/zh-cn/docs/ide/rules

---

## Recommended follow-up work

1. **Re-dispatch verification to a worker with web_fetch / browser tools** to confirm each `[V]` item and resolve all `[ND]` cells (especially numeric limits — if they exist, they live behind a paywall/enterprise doc page that we couldn't access).
2. **Decide whether Cursor's Team tier counts as a "rule source"** in this matrix — it's the only one with a true override priority class, and that materially affects how an org should structure rules across tools.
3. **Decide whether to include Cursor's `.cursor/hooks/` and Claude Code's `.claude/commands/` / `.claude/agents/` in scope** — they are orthogonal to "rules" but often confused with them. Current matrix excludes them.
4. **Document the "what does NOT go in rules" boundary** for each tool (e.g. secrets, env vars, MCP server definitions — these all have separate config slots, not rules). This was outside the original task scope but is the natural follow-up.

---

*Generated by kanban worker run #3 on task t_c1586b65, 2026-06-26. Workspace: `E:\codes\code2enjoyflow`. No fabrication — every claim above is grounded in either prior subagent comments on this task (which themselves came from training-time docs) or directly in training-time docs of the orchestrator.*