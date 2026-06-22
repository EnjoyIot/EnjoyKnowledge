//! End-to-end integration tests
//!
//! 测试完整链路。当前标记为 #[ignore]，随功能实现逐步激活。
//!
//! 激活顺序: init → search+record → doctor

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

fn enjoyknowledge() -> Command {
    Command::cargo_bin("enjoyknowledge").unwrap()
}

fn init_project(dir: &Path) {
    enjoyknowledge().arg("init").arg(dir.to_str().unwrap()).assert().success();
}

#[test]
#[ignore = "Phase 1: init 实现后激活"]
fn init_creates_directory_structure() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);
    assert!(root.join(".enjoyknowledge/knowledge-base/project").exists());
    assert!(root.join(".enjoyknowledge/knowledge-tasks").exists());
    assert!(root.join("AGENTS.md").exists());
}

#[test]
#[ignore = "Phase 2: record + search 实现后激活"]
fn record_and_search_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .args(["record", "gotcha", "--tag", "excel", "--content", "t_export_record 无 status 字段"])
        .current_dir(root)
        .assert()
        .success();

    enjoyknowledge()
        .args(["search", "status", "--class", "gotchas"])
        .current_dir(root)
        .assert()
        .success()
        .stdout(predicate::str::contains("GOTCHAS.md"))
        .stdout(predicate::str::contains("t_export_record"));
}

#[test]
#[ignore = "Phase 2: search 实现后激活"]
fn search_no_match_shows_empty_message() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .args(["search", "xyz_nonexistent_123"])
        .current_dir(root)
        .assert()
        .success()
        .stdout(predicate::str::contains("无匹配结果"));
}

#[test]
#[ignore = "Phase 2: record 实现后激活"]
fn record_gotcha_with_task_routes_to_knowledge_tasks() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .args([
            "record",
            "gotcha",
            "--task",
            "REQ-042",
            "--tag",
            "export",
            "--content",
            "异步导出超时",
        ])
        .current_dir(root)
        .assert()
        .success();

    assert!(root.join(".enjoyknowledge/knowledge-tasks/REQ-042/gotchas.md").exists());
}

#[test]
#[ignore = "Phase 3: doctor 实现后激活"]
fn doctor_reports_health() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    init_project(root);

    enjoyknowledge()
        .arg("doctor")
        .current_dir(root)
        .assert()
        .success()
        .stdout(predicate::str::contains("健康"));
}
