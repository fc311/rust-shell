use super::*;
use std::io::Cursor;

#[cfg(test)]
mod prompt_tests {
    use super::*;
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
}

#[cfg(test)]
mod exit_command_tests {
    use super::*;
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
}

#[cfg(test)]
mod version_command_tests {
    use super::*;
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
}

#[cfg(test)]
mod echo_command_tests {
    use super::*;

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

    #[test]
    fn test_repl_handles_empty_echo_command() {
        let input = Cursor::new("echo\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("echo: no arguments provided"));
    }
}

#[cfg(test)]
mod type_command_tests {
    use super::*;

    #[test]
    fn test_repl_handles_type_command() {
        let input = Cursor::new("type echo\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("echo is a shell builtin"));
    }

    #[test]
    fn test_repl_handles_type_type_command() {
        let input = Cursor::new("type type\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("type is a shell builtin"));
    }

    #[test]
    fn test_repl_handles_type_no_args() {
        let input = Cursor::new("type\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("type: no arguments provided"));
    }

    #[test]
    fn test_repl_handles_type_invalid_command() {
        let input = Cursor::new("type invalid_command\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("invalid_command: not found"));
    }
}

#[cfg(test)]
mod type_path_scan_tests {
    use super::*;

    #[test]
    fn test_repl_handles_type_path_scan_command_found_in_path() {
        // mock PATH with a temporary directory
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();

        // debug the temp path
        println!("Temporary PATH: {}", temp_path);

        // set the PATH environment variable to include the temp directory
        std::env::set_var("PATH", temp_path);

        // create a dummy executable named `ls` in the temp directory created above
        // this executable only exists for the test, no content needed
        std::fs::write(temp_dir.path().join("ls"), "").expect("Failed to create dummy ls");

        // debug the existence of the dummy executable
        println!(
            "Dummy ls executable exists?: {}",
            temp_dir.path().join("ls").exists()
        );

        // now setup and run the test
        let input = Cursor::new("type ls\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains(&format!("ls is {}/ls", temp_path)));
    }

    #[test]
    fn test_repl_handles_type_path_scan_command_not_in_path() {
        // Mock PATH with an empty directory
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();
        std::env::set_var("PATH", temp_path);

        let input = Cursor::new("type nonexistent\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("nonexistent: not found"));
    }
}
