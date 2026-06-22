---
name: omc-reference
description: "OMC agent catalog, available tools, team pipeline routing, commit protocol, and skills registry. Auto-loads when delegating to agents, using OMC tools, orchestrating teams, making commits, or invoking skills."
user-invocable: false
omc-full-body: "../../skill-bodies/omc-reference/SKILL.md"
---

<!-- OMC:COMPACT-PLUGIN-SKILL -->

# omc-reference

This is a compact Codex plugin registry shim. It keeps startup skill descriptions small while preserving the full OMC skill body for on-demand invocation.

When this skill is invoked, read and follow the full bundled instructions from the active plugin root:

`${CLAUDE_PLUGIN_ROOT:-${OMC_PLUGIN_ROOT}}/skill-bodies/omc-reference/SKILL.md`

The plugin root is the directory containing both `skills/` and `skill-bodies/`. Do not resolve `skill-bodies/omc-reference/SKILL.md` under this shim's `skills/omc-reference/` directory; `skill-bodies/` is a direct child of the plugin root. The same archived body path is recorded in frontmatter as `omc-full-body: ../../skill-bodies/omc-reference/SKILL.md` for hosts that understand plugin-root-relative metadata.
