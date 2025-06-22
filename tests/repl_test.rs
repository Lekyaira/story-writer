use assert_cmd::prelude::*;
use assert_cmd::Command;

#[test]
fn run_repl_exit_immediately() {
    let mut cmd = Command::cargo_bin("ai-chat").unwrap();
    cmd.env("OLLAMA_HOST", "localhost");
    cmd.env("OLLAMA_PORT", "1234");
    cmd.write_stdin(":exit\n");
    cmd.assert().success();
}
