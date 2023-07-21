use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn temp_to_f() -> TestResult {
let expected = "Temperature in degree fahrenheit: 212\n";
let mut cmd = Command::cargo_bin("temp_to_f")?;
cmd.arg("100").assert().success().stdout(expected);
Ok(())
}