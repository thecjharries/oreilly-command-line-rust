use assert_cmd::Command;

#[test]
fn test_true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
