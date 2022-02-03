use assert_cmd::Command;

#[test]
fn test_false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
