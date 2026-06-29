//! End-to-end integration tests for enjoyknowledge.
//!
//! Tests the full flow: init -> add -> grep -> doctor.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

fn enjoyknowledge() -> Command {
    Command::cargo_bin("enjoyknowledge").unwrap()
}

fn init_project(dir: &Path) {
    enjoyknowledge().arg("init").arg(dir.to_str().unwrap()).assert().success();
}

// ---- Phase 1: init ----

#[test]
fn init_creates_directory_structure() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    assert!(root.join(".enjoyknowledge").exists());
    assert!(root.join(".enjoyknowledge/architecture").exists());
    assert!(root.join(".enjoyknowledge/gotcha").exists());
    assert!(root.join(".enjoyknowledge/pattern").exists());
    assert!(root.join(".enjoyknowledge/business").exists());
    assert!(root.join(".enjoyknowledge/decision").exists());
    assert!(root.join("knowledge-tasks").exists());
    assert!(root.join("AGENTS.md").exists());
}

// ---- Phase 2: add + grep ----

#[test]
fn add_and_grep_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // Add a knowledge entry
    enjoyknowledge()
        .args(["add", "gotcha/export.md", "## export timeout\n- API times out over 100k rows"])
        .current_dir(root)
        .assert()
        .success();

    // Grep should find it
    enjoyknowledge()
        .args(["grep", "timeout"])
        .current_dir(root)
        .assert()
        .stdout(predicate::str::contains("export.md"));
}

#[test]
fn grep_no_match_exits_nonzero() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge().args(["grep", "xyz_nonexistent_123"]).current_dir(root).assert().code(1);
}

#[test]
fn add_creates_file_under_category() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .args(["add", "gotcha/export.md", "## Excel OOM\n- SXSSFWorkbook unclosed causes OOM"])
        .current_dir(root)
        .assert()
        .success();

    assert!(root.join(".enjoyknowledge/gotcha/export.md").exists());
}

// ---- Phase 3: doctor ----

#[test]
fn doctor_reports_clean_on_fresh_init() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge().arg("doctor").current_dir(root).assert().success();
}

// ---- Phase 4: --ai generic ----

#[test]
fn init_ai_generic_creates_enjoyknowledge_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    enjoyknowledge().args(["init", "--ai", "generic"]).current_dir(root).assert().success();

    assert!(root.join("enjoyknowledge.md").exists());
    let content = std::fs::read_to_string(root.join("enjoyknowledge.md")).unwrap();
    assert!(content.contains("enjoyknowledge"));
    assert!(content.contains("Shared AI Context"));
}

// ---- Phase 5: --template list ----

#[test]
fn init_template_list_runs() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    enjoyknowledge().args(["init", "--template", "list"]).current_dir(root).assert().success();
}

// ---- Phase 6: doctor --ci ----

#[test]
fn doctor_ci_exits_nonzero_on_warnings() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // Create a stale file (timestamp > 180 days) to trigger a staleness Warning
    let ek = root.join(".enjoyknowledge");
    std::fs::write(
        ek.join("old.md"),
        "---\ndescription: Old entry\ntags: [test]\ntimestamp: 2025-01-15\n---\n\n## Old\n- content\n",
    )
    .unwrap();

    // doctor (non-ci) should succeed with a warning
    enjoyknowledge().arg("doctor").current_dir(root).assert().success();

    // doctor --ci should fail with non-zero on any warning
    enjoyknowledge().args(["doctor", "--ci"]).current_dir(root).assert().code(3);
}

// ---- v0.4: Enhanced init ----

#[test]
fn init_creates_stage_directory_structure() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    assert!(root.join(".enjoyknowledge_stage").exists());
    assert!(root.join(".enjoyknowledge_stage/tasks").exists());
    assert!(root.join(".enjoyknowledge_stage/drafts").exists());
    assert!(root.join(".enjoyknowledge_stage/.archive/tasks").exists());
}

#[test]
fn init_creates_8_stage_templates() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let td = root.join(".enjoyknowledge_stage/tasks/_template");
    assert!(td.exists());
    for name in &[
        "summary.md",
        "requirements.md",
        "design.md",
        "plan.md",
        "changes.md",
        "tests.md",
        "delivery.md",
        "review.md",
    ] {
        assert!(td.join(name).exists(), "missing {name}");
    }
}

#[test]
fn init_generates_ek_agents_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/AGENTS.md")).unwrap();
    assert!(content.contains("enjoyknowledge 知识库"));
    assert!(content.contains("读写原则"));
}

#[test]
fn init_generates_stage_agents_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge_stage/AGENTS.md")).unwrap();
    assert!(content.contains("enjoyknowledge 任务执行"));
    assert!(content.contains("任务执行流程"));
    assert!(content.contains("drafts/"));
}

#[test]
fn init_generates_stage_defaults_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let path = root.join(".enjoyknowledge_stage/_meta/stage-defaults.md");
    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("Stage Default Directories"));
    assert!(content.contains("| name | purpose | files |"));
}

#[test]
fn init_does_not_overwrite_user_stage_agents_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // User edits stage AGENTS.md
    let user_content = "# My Custom Stage Rules\n\nUser edited this file.";
    std::fs::write(root.join(".enjoyknowledge_stage/AGENTS.md"), user_content).unwrap();

    // Re-init should NOT overwrite
    init_project(root);

    let actual = std::fs::read_to_string(root.join(".enjoyknowledge_stage/AGENTS.md")).unwrap();
    assert_eq!(actual, user_content);
}

#[test]
fn init_does_not_overwrite_user_stage_defaults_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // User edits stage-defaults.md
    let user_content = "# My custom stage defaults\n\nCustomized.";
    std::fs::write(root.join(".enjoyknowledge_stage/_meta/stage-defaults.md"), user_content)
        .unwrap();

    // Re-init should NOT overwrite
    init_project(root);

    let actual =
        std::fs::read_to_string(root.join(".enjoyknowledge_stage/_meta/stage-defaults.md"))
            .unwrap();
    assert_eq!(actual, user_content);
}

#[test]
fn init_reads_stage_defaults_for_directory_creation() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // First create a custom stage-defaults.md before init
    let stage_dir = root.join(".enjoyknowledge_stage");
    std::fs::create_dir_all(stage_dir.join("_meta")).unwrap();
    let custom = "# Stage Default Directories\n\n## Default Directories\n\n| name | purpose | files |\n|------|---------|-------|\n| `tasks` | Active task records | summary |\n| `drafts` | Knowledge drafts | (user writes freely) |\n| `experiments` | Extra experiments dir | (user writes freely) |\n";
    std::fs::write(stage_dir.join("_meta/stage-defaults.md"), custom).unwrap();

    init_project(root);

    // Custom dir from stage-defaults.md should be created
    assert!(root.join(".enjoyknowledge_stage/experiments").exists());
    // Backward compat: .archive/tasks still created
    assert!(root.join(".enjoyknowledge_stage/.archive/tasks").exists());
}

#[test]
fn init_creates_11_kind_directories() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let ek = root.join(".enjoyknowledge");
    for kind in &[
        "architecture",
        "business",
        "command",
        "context",
        "decision",
        "gotcha",
        "pattern",
        "rule",
        "contract",
        "convention",
        "template",
    ] {
        assert!(ek.join(kind).exists(), "missing kind dir: {kind}");
    }
}

#[test]
fn init_updates_gitignore_when_present() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    std::fs::write(root.join(".gitignore"), "target/\n").unwrap();

    init_project(root);

    let content = std::fs::read_to_string(root.join(".gitignore")).unwrap();
    assert!(content.contains(".enjoyknowledge_stage/tasks/*/changes.md"));
}

// ---- v0.4: Promote ----

#[test]
fn promote_draft_to_gotcha() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // Write a draft
    std::fs::write(
        root.join(".enjoyknowledge_stage/drafts/utf8-windows.md"),
        "## Windows UTF-8 encoding\n- CP_UTF8 flag issue on Windows 10\n",
    )
    .unwrap();

    enjoyknowledge()
        .args(["promote", "utf8-windows.md", "--to", "gotcha", "--author", "enjoy"])
        .current_dir(root)
        .assert()
        .success();

    let target = root.join(".enjoyknowledge/gotcha/utf8-windows.md");
    assert!(target.exists());
    let content = std::fs::read_to_string(target).unwrap();
    assert!(content.contains("id: utf8-windows"));
    assert!(content.contains("kind: gotcha"));
    assert!(content.contains("author: enjoy"));
    assert!(content.contains("Windows UTF-8 encoding"));
}

#[test]
fn promote_adds_promoted_marker() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    std::fs::write(root.join(".enjoyknowledge_stage/drafts/my-draft.md"), "## My Draft\n- test\n")
        .unwrap();

    enjoyknowledge()
        .args(["promote", "my-draft.md", "--to", "architecture", "--author", "enjoy"])
        .current_dir(root)
        .assert()
        .success();

    let draft =
        std::fs::read_to_string(root.join(".enjoyknowledge_stage/drafts/my-draft.md")).unwrap();
    assert!(draft.contains("[PROMOTED]"));
}

#[test]
fn promote_missing_draft_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .args(["promote", "nonexistent.md", "--to", "gotcha"])
        .current_dir(root)
        .assert()
        .code(1);
}

// ---- v0.4: Stage Clean ----

#[test]
fn stage_clean_no_archive_is_noop() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge().args(["stage", "clean", "--force"]).current_dir(root).assert().success();
}

#[test]
fn stage_clean_dry_run_lists_tasks() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // Create an old task in archive
    let archive = root.join(".enjoyknowledge_stage/.archive/tasks/old-task");
    std::fs::create_dir_all(&archive).unwrap();
    std::fs::write(archive.join("summary.md"), "old task summary").unwrap();

    enjoyknowledge()
        .args(["stage", "clean", "--dry-run", "--older-than", "0"])
        .current_dir(root)
        .assert()
        .success();
}

#[test]
fn stage_clean_force_removes_old_tasks() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let archive = root.join(".enjoyknowledge_stage/.archive/tasks/old-task");
    std::fs::create_dir_all(&archive).unwrap();
    std::fs::write(archive.join("summary.md"), "old task").unwrap();

    enjoyknowledge()
        .args(["stage", "clean", "--force", "--older-than", "0"])
        .current_dir(root)
        .assert()
        .success();

    assert!(!archive.exists());
}

#[test]
fn stage_clean_without_force_does_not_delete() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let archive = root.join(".enjoyknowledge_stage/.archive/tasks/old-task");
    std::fs::create_dir_all(&archive).unwrap();
    std::fs::write(archive.join("summary.md"), "old task").unwrap();

    enjoyknowledge()
        .args(["stage", "clean", "--older-than", "0"])
        .current_dir(root)
        .assert()
        .success();

    // Without --force, tasks should NOT be deleted
    assert!(archive.exists());
}

// ---- v0.4.7: AGENTS.md 不覆盖 + 默认模板 + 任务执行流程 ----

#[test]
fn init_does_not_overwrite_user_ek_agents_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // User edits ek AGENTS.md with a marker
    let user_content = "# My KB Rules\n\nUSER-MARKER-12345\n\nCustom rules here.\n";
    std::fs::write(root.join(".enjoyknowledge/AGENTS.md"), user_content).unwrap();

    // Re-init should NOT overwrite
    init_project(root);

    let actual = std::fs::read_to_string(root.join(".enjoyknowledge/AGENTS.md")).unwrap();
    assert_eq!(actual, user_content, "user marker should be preserved");
}

#[test]
fn init_creates_default_ek_agents_md_with_static_dir_and_briefly_flow() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Remove any existing AGENTS.md first so init creates a fresh default
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/AGENTS.md")).unwrap();
    // Static directory description present
    assert!(content.contains("architecture/"), "should describe architecture dir");
    assert!(content.contains("gotcha/"), "should describe gotcha dir");
    assert!(content.contains("读写原则"), "should have read/write principles");
    assert!(content.contains("常见操作"), "should have common operations table");
    // Briefly mentions flow (1-2 sentences: the skills/ reference and init no-overwrite note)
    assert!(
        content.contains("skills/") && content.contains("v0.4.8"),
        "should briefly mention flow skills path"
    );
}

#[test]
fn init_creates_stage_agents_md_with_task_execution_workflow() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge_stage/AGENTS.md")).unwrap();
    assert!(content.contains("enjoyknowledge 任务执行"), "should have stage title");
    assert!(content.contains("任务执行流程"), "should have task execution workflow");
    assert!(content.contains("drafts/"), "should mention drafts dir");
    assert!(content.contains("tasks/"), "should mention tasks dir");
    assert!(content.contains(".archive/"), "should mention archive dir");
    assert!(content.contains("ek promote"), "should mention promote command");
    assert!(content.contains("ek stage clean"), "should mention stage clean command");
}

// ---- v0.4.8: Skills workflow files ----

#[test]
fn init_creates_skills_directory_with_4_flows_and_index() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let skills_dir = root.join(".enjoyknowledge/skills");
    assert!(skills_dir.exists(), "skills directory should exist");

    for name in &["coding.md", "research.md", "review.md", "design.md", "README.md"] {
        assert!(skills_dir.join(name).exists(), "missing skills file: {name}");
    }
}

#[test]
fn init_does_not_overwrite_user_skills_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    // User edits coding.md
    let user_content = "# My Custom Coding Flow\n\nUSER-MARKER-12345\n\nCustom rules here.\n";
    std::fs::write(root.join(".enjoyknowledge/skills/coding.md"), user_content).unwrap();

    // Re-init should NOT overwrite
    init_project(root);

    let actual = std::fs::read_to_string(root.join(".enjoyknowledge/skills/coding.md")).unwrap();
    assert_eq!(actual, user_content, "user marker should be preserved");
}

#[test]
fn init_skills_coding_md_has_correct_frontmatter() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/skills/coding.md")).unwrap();
    assert!(content.contains("name: enjoyknowledge-flow-coding"));
    assert!(content.contains("编码工作流"));
    assert!(content.contains("Step-by-step"));
    assert!(content.contains("File Reading Order"));
    assert!(content.contains("File Writing Order"));
    assert!(content.contains("Common Patterns"));
    assert!(content.contains("Pitfalls"));
}

#[test]
fn init_skills_research_md_has_correct_frontmatter() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/skills/research.md")).unwrap();
    assert!(content.contains("name: enjoyknowledge-flow-research"));
    assert!(content.contains("调研工作流"));
    assert!(content.contains("Step-by-step"));
}

#[test]
fn init_skills_review_md_has_correct_frontmatter() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/skills/review.md")).unwrap();
    assert!(content.contains("name: enjoyknowledge-flow-review"));
    assert!(content.contains("复盘工作流"));
    assert!(content.contains("Step-by-step"));
}

#[test]
fn init_skills_design_md_has_correct_frontmatter() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    let content = std::fs::read_to_string(root.join(".enjoyknowledge/skills/design.md")).unwrap();
    assert!(content.contains("name: enjoyknowledge-flow-design"));
    assert!(content.contains("设计工作流"));
    assert!(content.contains("Step-by-step"));
}
