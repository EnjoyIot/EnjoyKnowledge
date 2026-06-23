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
    assert!(root.join(".enjoyknowledge/gotchas").exists());
    assert!(root.join(".enjoyknowledge/patterns").exists());
    assert!(root.join(".enjoyknowledge/business").exists());
    assert!(root.join(".enjoyknowledge/decisions").exists());
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
        .args(["add", "gotchas/export.md", "## export timeout\n- API times out over 100k rows"])
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
        .args(["add", "gotchas/export.md", "## Excel OOM\n- SXSSFWorkbook unclosed causes OOM"])
        .current_dir(root)
        .assert()
        .success();

    assert!(root.join(".enjoyknowledge/gotchas/export.md").exists());
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

    // Create a file with missing description to trigger a warning
    let ek = root.join(".enjoyknowledge").join("gotchas");
    std::fs::create_dir_all(&ek).unwrap();
    std::fs::write(
        ek.join("test.md"),
        "---\ntitle: Test Gotcha\ntimestamp: 2026-06-23\n---\n\n## Test\n- A simple gotcha\n",
    )
    .unwrap();

    // doctor (non-ci) should succeed with a warning
    enjoyknowledge().arg("doctor").current_dir(root).assert().success();

    // doctor --ci should fail with non-zero on any warning
    enjoyknowledge().args(["doctor", "--ci"]).current_dir(root).assert().code(3);
}
