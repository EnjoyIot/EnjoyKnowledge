# Claude Code Rules / Instructions Mechanism Brief

Research target: concrete facts the rule-mechanism comparison matrix needs about how Claude Code loads, scopes, prioritises, formats, and limits its memory of instructions. All quotes and paths come from Anthropic official docs and the official anthropics/claude-code repo. Inline URLs cite the exact page.

Cited primary sources:
- Memory / CLAUDE.md https://docs.anthropic.com/en/docs/claude-code/memory
- Settings scopes and precedence https://docs.anthropic.com/en/docs/claude-code/settings
- Hooks reference (event names, JSON shapes) https://docs.anthropic.com/en/docs/claude-code/hooks
- Skills / slash commands https://docs.anthropic.com/en/docs/claude-code/slash-commands
- Sub-agents https://docs.anthropic.com/en/docs/claude-code/sub-agents
- Monorepos / large codebases guide https://docs.anthropic.com/en/docs/claude-code/monorepos
- Main Claude Code index https://docs.anthropic.com/en/docs/claude-code
- Official repo README https://github.com/anthropics/claude-code

---

## 1. File locations and naming conventions

Claude Code has two complementary memory systems loaded at the start of every conversation: CLAUDE.md files (instructions you write) and auto memory (notes Claude writes itself) (https://docs.anthropic.com/en/docs/claude-code/memory).

### 1.1 Hard-coded file names the loader scans for

- CLAUDE.md -- primary project / user / managed instruction file.
- CLAUDE.local.md -- personal project-specific overrides (gitignore it). "For private per-project preferences that should not be checked into version control, create a CLAUDE.local.md at the project root. It loads alongside CLAUDE.md and is treated the same way." (https://docs.anthropic.com/en/docs/claude-code/memory)
- .claude/CLAUDE.md -- alternate project location, treated identically to ./CLAUDE.md. "A project CLAUDE.md can be stored in either ./CLAUDE.md or ./.claude/CLAUDE.md." (https://docs.anthropic.com/en/docs/claude-code/memory)
- .claude/rules/*.md -- modular topic-specific rule files, loaded recursively. "All .md files are discovered recursively, so you can organize rules into subdirectories like frontend/ or backend/." (https://docs.anthropic.com/en/docs/claude-code/memory)
- ~/.claude/CLAUDE.md and ~/.claude/rules/*.md -- user-scope counterparts.
- AGENTS.md -- NOT read by Claude Code directly. "Claude Code reads CLAUDE.md, not AGENTS.md. If your repository already uses AGENTS.md for other coding agents, create a CLAUDE.md that imports it..." Workaround pattern: a one-line CLAUDE.md containing @AGENTS.md, or a symlink (ln -s AGENTS.md CLAUDE.md). On Windows use the import because symlinks need Admin/Dev Mode. (https://docs.anthropic.com/en/docs/claude-code/memory)
- MEMORY.md -- auto-memory entrypoint at ~/.claude/projects/<project>/memory/MEMORY.md. Same first-200-lines-or-25KB rule as the project MEMORY.md of a subagent (~/.claude/agent-memory/<name>/MEMORY.md). (https://docs.anthropic.com/en/docs/claude-code/memory, https://docs.anthropic.com/en/docs/claude-code/sub-agents)

### 1.2 Managed / org-wide paths (cannot be overridden)

| OS | Managed CLAUDE.md path |
|---|---|
| macOS | /Library/Application Support/ClaudeCode/CLAUDE.md |
| Linux / WSL | /etc/claude-code/CLAUDE.md |
| Windows | C:\Program Files\ClaudeCode\CLAUDE.md |

Source: https://docs.anthropic.com/en/docs/claude-code/memory and https://docs.anthropic.com/en/docs/claude-code/settings

On Windows the user-home roots (~/.claude/...) resolve to %USERPROFILE%\.claude. (https://docs.anthropic.com/en/docs/claude-code/settings)

### 1.3 Hierarchy / traversal order

"Claude Code reads CLAUDE.md files by walking up the directory tree from your current working directory, checking each directory along the way for CLAUDE.md and CLAUDE.local.md files... All discovered files are concatenated into context rather than overriding each other. Across the directory tree, content is ordered from the filesystem root down to your working directory... Within each directory, CLAUDE.local.md is appended after CLAUDE.md." (https://docs.anthropic.com/en/docs/claude-code/memory)

Subdirectory CLAUDE.md files below the working directory are lazy-loaded: "Claude also discovers CLAUDE.md and CLAUDE.local.md files in subdirectories under your current working directory. Instead of loading them at launch, they are included when Claude reads files in those subdirectories." (https://docs.anthropic.com/en/docs/claude-code/memory)

Lazy-load is also how path-scoped .claude/rules/*.md files trigger: only when Claude reads a file matching the rule paths: glob. (https://docs.anthropic.com/en/docs/claude-code/memory)

### 1.4 @path/to/import syntax for inlining files

"CLAUDE.md files can import additional files using @path/to/import syntax. Imported files are expanded and loaded into context at launch alongside the CLAUDE.md that references them. Both relative and absolute paths are allowed. Relative paths resolve relative to the file containing the import, not the working directory. Imported files can recursively import other files, with a maximum depth of four hops. Import parsing skips Markdown code spans and fenced code blocks. To mention a path in your CLAUDE.md without importing it, wrap it in backticks: writing @README keeps the text literal, while @README outside backticks imports the file." (https://docs.anthropic.com/en/docs/claude-code/memory)

On first encounter per project Claude shows an approval dialog listing the imported files. Decline -> imports stay disabled for that project. (https://docs.anthropic.com/en/docs/claude-code/memory)

---

## 2. Scope rules (project vs user vs org, local vs shared)

### 2.1 The four explicit scopes, in load order (broadest to most specific)

| Scope | Location | Shared with |
|---|---|---|
| Managed policy | /Library/Application Support/ClaudeCode/CLAUDE.md (mac) / /etc/claude-code/CLAUDE.md (linux/WSL) / C:\Program Files\ClaudeCode\CLAUDE.md (win) | All users in organisation |
| User instructions | ~/.claude/CLAUDE.md | Just you, all projects |
| Project instructions | ./CLAUDE.md or ./.claude/CLAUDE.md | Team via source control |
| Local instructions | ./CLAUDE.local.md | Just you, current project (gitignored) |

"A project instruction appears in context after a user instruction." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 2.2 Additional directory support via --add-dir

"By default, CLAUDE.md files from these directories are not loaded. To also load memory files from additional directories, set the CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD environment variable: CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config. This loads CLAUDE.md, .claude/CLAUDE.md, .claude/rules/*.md, and CLAUDE.local.md from the additional directory." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 2.3 Subdirectory scope is implicit (auto-load on read)

There is no explicit "package scope"; subdirectories under the cwd get their own CLAUDE.md that loads when Claude first reads a file in that directory. (https://docs.anthropic.com/en/docs/claude-code/memory)

### 2.4 Sub-agents carry their own scope

A sub-agent declared with user scope gets ~/.claude/agent-memory/<name-of-agent>/MEMORY.md; with project scope it gets .claude/agent-memory/<name-of-agent>/ (shareable via VCS); with None no memory persists. (https://docs.anthropic.com/en/docs/claude-code/sub-agents)

---

## 3. Priority / override behaviour when multiple sources coexist

### 3.1 CLAUDE.md is context, not enforced configuration

"Both [CLAUDE.md and auto memory] are loaded at the start of every conversation. Claude treats them as context, not enforced configuration. To block an action regardless of what Claude decides, use a PreToolUse hook instead." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 3.2 Within a session: concat, not override

"All discovered files are concatenated into context rather than overriding each other." Order: filesystem root to cwd; CLAUDE.local.md after CLAUDE.md per directory. (https://docs.anthropic.com/en/docs/claude-code/memory)

### 3.3 Conflicting rules: Claude picks arbitrarily

"If two rules contradict each other, Claude may pick one arbitrarily. Review your CLAUDE.md files, nested CLAUDE.md files in subdirectories, and .claude/rules/ periodically to remove outdated or conflicting instructions." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 3.4 Settings-level priority (separate from CLAUDE.md itself)

For settings.json keys (which can carry claudeMd and claudeMdExcludes), the precedence order from highest to lowest is:
1. Managed (cannot be overridden)
2. Command-line arguments
3. Local (.claude/settings.local.json)
4. Project (.claude/settings.json)
5. User (~/.claude/settings.json)

(https://docs.anthropic.com/en/docs/claude-code/settings)

### 3.5 Managed CLAUDE.md precedence over project CLAUDE.md

"The managed CLAUDE.md... loads before user and project CLAUDE.md." But importantly: claudeMd is only honoured in managed/policy settings -- setting it in user/project/local settings has no effect. (https://docs.anthropic.com/en/docs/claude-code/memory, https://docs.anthropic.com/en/docs/claude-code/settings)

### 3.6 Exclusions: claudeMdExcludes

```jsonc
// .claude/settings.local.json
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

"Patterns are matched against absolute file paths using glob syntax. You can configure claudeMdExcludes at any settings layer: user, project, local, or managed policy. Arrays merge across layers. Managed policy CLAUDE.md files cannot be excluded." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 3.7 Slash-command / skill override precedence

"When skills share the same name across levels, enterprise overrides personal, and personal overrides project. A skill at any of these levels also overrides a bundled skill with the same name." If a skill and a legacy .claude/commands/*.md share the same name, the skill wins. (https://docs.anthropic.com/en/docs/claude-code/slash-commands)

---

## 4. Format constraints (markdown? frontmatter? imports? slash-commands? hooks? sub-agents?)

### 4.1 File format: plain Markdown

"CLAUDE.md files are markdown files that give Claude persistent instructions for a project, your personal workflow, or your entire organization. You write these files in plain text; Claude reads them at the start of every session." (https://docs.anthropic.com/en/docs/claude-code/memory)

HTML block comments are stripped before injection: "Block-level HTML comments (<!-- maintainer notes -->) in CLAUDE.md files are stripped before the content is injected into Claude context." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 4.2 Frontmatter -- only .claude/rules/*.md uses it (and only one key)

Path-scoped rules use YAML frontmatter with a single paths: key:

```yaml
---
paths:
  - "src/api/**/*.ts"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---

# API Development Rules
- All API endpoints must include input validation
```

"Rules without a paths field are loaded unconditionally and apply to all files. Path-scoped rules trigger when Claude reads files matching the pattern, not on every tool use. Use glob patterns in the paths field..." (https://docs.anthropic.com/en/docs/claude-code/memory)

CLAUDE.md itself does not use frontmatter for behaviour toggling; the doc only shows description: frontmatter inside skills/commands (see 4.4).

### 4.3 @path/to/import inline inclusion

Already detailed in section 1.4. Recursion depth limit: 4 hops max. Parsing skips fenced code blocks and inline code spans. (https://docs.anthropic.com/en/docs/claude-code/memory)

### 4.4 Slash commands / Skills (/command)

"Custom commands have been merged into skills. A file at .claude/commands/deploy.md and a skill at .claude/skills/deploy/SKILL.md both create /deploy and work the same way." (https://docs.anthropic.com/en/docs/claude-code/slash-commands)

Supported description: frontmatter (example for skills):

```yaml
---
description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
---
```

Built-in commands include /help, /compact, /init, /memory, /bug. Bundled skills: /code-review, /batch, /debug, /loop, /claude-api. (https://docs.anthropic.com/en/docs/claude-code/slash-commands)

### 4.5 Hooks (enforced, deterministic lifecycle triggers)

Defined as JSON in any settings file (user/project/local/managed). Three levels of nesting: event to matcher to array of handlers. Hook types: shell commands, HTTP endpoints, or LLM prompts. (https://docs.anthropic.com/en/docs/claude-code/hooks)

Full event list (every Claude Code lifecycle hook):
- Once per session: SessionStart, SessionEnd
- Once per turn: UserPromptSubmit, Stop, StopFailure
- Per tool call: PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest, PermissionDenied
- Plus: Notification, PostCompact, PostToolBatch, TeammateIdle, TaskCreated, TaskCompleted, WorktreeCreate, WorktreeRemove, CwdChanged, FileChanged, InstructionsLoaded, SubagentStart, SubagentStop.

PreToolUse can block tool calls (returns "permissionDecision": "deny" or "allow"). (https://docs.anthropic.com/en/docs/claude-code/hooks)

The dedicated InstructionsLoaded hook fires when a CLAUDE.md or .claude/rules/*.md is loaded. JSON fields exposed:
- file_path -- absolute path
- memory_type -- "User" | "Project" | "Local" | "Managed"
- load_reason -- "session_start" | "nested_traversal" | "path_glob_match" | "include" | "compact"
- globs -- present only for path_glob_match loads
- trigger_file_path -- file whose read triggered this load
- parent_file_path -- parent instruction file for include loads

"It has no decision control. It cannot block or modify instruction loading." (https://docs.anthropic.com/en/docs/claude-code/hooks)

Matcher values for InstructionsLoaded are session_start, nested_traversal, path_glob_match, include, compact. (https://docs.anthropic.com/en/docs/claude-code/hooks)

### 4.6 Sub-agents

Declared as Markdown files in .claude/agents/<name>.md with YAML frontmatter (name, description, tools, model, etc.). The sub-agent gets its own system prompt and may carry its own auto-memory scope (user to ~/.claude/agent-memory/<name>/, project to .claude/agent-memory/<name>/, or None). (https://docs.anthropic.com/en/docs/claude-code/sub-agents)

### 4.7 --append-system-prompt for true system-prompt injection

"CLAUDE.md content is delivered as a user message after the system prompt, not as part of the system prompt itself." To force a setting onto the system prompt itself, use the CLI flag --append-system-prompt (per-invocation; better for scripts than interactive use). (https://docs.anthropic.com/en/docs/claude-code/memory)

---

## 5. Limits (file size, file count, context budget, glob syntax)

### 5.1 Size budget per CLAUDE.md

"Target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce adherence. If your instructions are growing large, use path-scoped rules so instructions load only when Claude works with matching files. You can also split content into imports for organization, though imported files still load and enter the context window at launch." (https://docs.anthropic.com/en/docs/claude-code/memory)

The 200-line figure is repeated in the troubleshooting section: "Files over 200 lines consume more context and may reduce adherence." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 5.2 Auto-memory index (MEMORY.md) cap

"The first 200 lines of MEMORY.md, or the first 25KB, whichever comes first, are loaded at the start of every conversation. Content beyond that threshold is not loaded at session start... This limit applies only to MEMORY.md. CLAUDE.md files are loaded in full regardless of length, though shorter files produce better adherence. Topic files like debugging.md or patterns.md are not loaded at startup. Claude reads them on demand using its standard file tools when it needs the information." (https://docs.anthropic.com/en/docs/claude-code/memory)

Sub-agent memory uses the same 200 lines / 25KB limit: "The subagent system prompt also includes the first 200 lines or 25KB of MEMORY.md in the memory directory, whichever comes first, with instructions to curate MEMORY.md if it exceeds that limit." (https://docs.anthropic.com/en/docs/claude-code/sub-agents)

### 5.3 Import recursion depth

"Imported files can recursively import other files, with a maximum depth of four hops." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 5.4 Symlink loop handling

"The .claude/rules/ directory supports symlinks, so you can maintain a shared set of rules and link them into multiple projects. Symlinks are resolved and loaded normally, and circular symlinks are detected and handled gracefully." (https://docs.anthropic.com/en/docs/claude-code/memory)

### 5.5 Behaviour after /compact

"Project-root CLAUDE.md survives compaction: after /compact, Claude re-reads it from disk and re-injects it into the session. Nested CLAUDE.md files in subdirectories are not re-injected automatically; they reload the next time Claude reads a file in that subdirectory." (https://docs.anthropic.com/en/docs/claude-code/memory)

InstructionsLoaded hook fires again with load_reason: "compact" after compaction. (https://docs.anthropic.com/en/docs/claude-code/hooks)

### 5.6 Number-of-files limit

The docs do not specify a hard cap on the number of CLAUDE.md / rules files. They prescribe operational patterns instead: split into .claude/rules/*.md, use claudeMdExcludes for monorepos, and keep each file under 200 lines. (claudeMdExcludes example with two globs is the closest thing to a documented limit pattern: https://docs.anthropic.com/en/docs/claude-code/memory)

### 5.7 Context window budget overall

CLAUDE.md files are loaded as context, not as configuration: "CLAUDE.md files are loaded into the context window at the start of every session, consuming tokens alongside your conversation." Path-scoped rules only load on a matching file read, so they do not burn the launch budget. (https://docs.anthropic.com/en/docs/claude-code/memory)

A separate doc, "Explore the context window" (https://docs.anthropic.com/en/docs/claude-code/context-window), tracks the overall budget per model and shows where CLAUDE.md loads relative to system prompt, MCP servers, skills, and conversation.

### 5.8 Toggling defaults

- Auto memory: on by default; toggle via /memory UI, autoMemoryEnabled: false in settings, or env CLAUDE_CODE_DISABLE_AUTO_MEMORY=1. (https://docs.anthropic.com/en/docs/claude-code/memory)
- claudeMdExcludes is honoured at user, project, local, and managed settings layers (but cannot exclude the managed CLAUDE.md itself). (https://docs.anthropic.com/en/docs/claude-code/memory)
- autoMemoryDirectory lets you point memory elsewhere; must be absolute or ~/-prefixed; only effective after the workspace trust dialog accepts the folder. (https://docs.anthropic.com/en/docs/claude-code/memory)

---

## Quick-reference numbers for the matrix

| Dimension | Value | Source |
|---|---|---|
| Per-CLAUDE.md soft size cap | 200 lines | memory doc, troubleshooting |
| Auto-memory MEMORY.md cap | 200 lines OR 25 KB (whichever first) | memory doc, sub-agents doc |
| Import recursion depth | 4 hops | memory doc |
| Managed CLAUDE.md paths | mac: /Library/Application Support/ClaudeCode/CLAUDE.md; linux/WSL: /etc/claude-code/CLAUDE.md; win: C:\Program Files\ClaudeCode\CLAUDE.md | memory doc |
| User CLAUDE.md path | ~/.claude/CLAUDE.md | memory doc |
| Project CLAUDE.md paths | ./CLAUDE.md OR ./.claude/CLAUDE.md | memory doc |
| Local CLAUDE.md path | ./CLAUDE.local.md (gitignored) | memory doc |
| Rules dir | .claude/rules/ (recursive, accepts subdirs) | memory doc |
| Frontmatter for path-scoping | single key paths: (YAML list of globs) | memory doc |
| HTML comments | block-level stripped from injected context | memory doc |
| AGENTS.md native read | NO -- must be @AGENTS.md-imported or symlinked | memory doc |
| Slash commands vs skills | merged; same name -> skill wins | slash-commands doc |
| Settings precedence | Managed > CLI > Local > Project > User | settings doc |
| claudeMd key honoured in | Managed/policy only | memory doc, settings doc |
| claudeMdExcludes can exclude managed? | NO | memory doc |
| Auto-memory default state | ON (since v2.1.59) | memory doc |
| Auto-memory toggle env var | CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 | memory doc |
| Auto-memory sub-agent paths | user: ~/.claude/agent-memory/<name>/; project: .claude/agent-memory/<name>/ | sub-agents doc |
| --add-dir env to load its CLAUDE.md | CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 | memory doc |
| InstructionsLoaded hook matcher values | session_start, nested_traversal, path_glob_match, include, compact | hooks doc |
| CLAUDE.md delivered as | user message (after system prompt) -- not enforced | memory doc |
| Path to enforce behaviour | PreToolUse hook (returns permissionDecision) | memory doc, hooks doc |
| CLI to put text in the actual system prompt | --append-system-prompt | memory doc |
| Compaction behaviour | root CLAUDE.md re-injected; nested not | memory doc |
