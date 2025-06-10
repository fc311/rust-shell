use super::*;
use std::io::Cursor;

#[test]
fn test_repl_prints_prompt_and_handles_input() {
    let input = Cursor::new("hello\nexit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

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
    assert_eq!(result.unwrap(), 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(!output_str.contains("command not found"));
}

#[test]
fn test_repl_handles_version_command() {
    let input = Cursor::new("version\nexit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(output_str.contains("Simple Shell v0.1.0"));
}

#[test]
fn test_repl_exits_on_exit_0_command() {
    let input = Cursor::new("exit 0\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(!output_str.contains("command not found"));
}

#[test]
fn test_repl_rejects_exit_with_non_zero_arg() {
    let input = Cursor::new("exit 42\nexit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(output_str.contains("exit 42: command not found"));
}

#[test]
fn test_repl_handles_echo_command() {
    let input = Cursor::new("echo Hello World\nexit\n");
    let mut output = Vec::new();

    let result = run_repl(input, &mut output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("$ "));
    assert!(output_str.contains("Hello World"));
    assert!(!output_str.contains("echo: no arguments provided"));
    assert!(!output_str.contains("command not found"));
}
