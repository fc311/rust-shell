use super::*;
use std::io::Cursor;

#[test]
fn test_repl_prints_prompt_and_handles_input() {
    let input = Cursor::new("hello\nexit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(output_str.contains("hello: command not found"));
}

#[test]
fn test_repl_exits_on_exit_command() {
    let input = Cursor::new("exit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(!output_str.contains("command not found"));
}
