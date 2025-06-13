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

        // set the PATH environment variable to include the temp directory
        std::env::set_var("PATH", temp_path);

        // create a dummy executable named `ls` in the temp directory created above
        // this executable only exists for the test, no content needed
        std::fs::write(temp_dir.path().join("ls"), "").expect("Failed to create dummy ls");

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

#[cfg(test)]
mod executable_tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_repl_runs_executable_in_path() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();
        std::env::set_var("PATH", temp_path);

        // Create a mock executable (shell script)
        let script_path = temp_dir.path().join("testcmd");
        fs::write(&script_path, "#!/bin/sh\necho 'Hello from testcmd'\n")
            .expect("Failed to create mock executable");
        // Make it executable
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))
            .expect("Failed to set executable permissions");

        let input = Cursor::new("testcmd\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("Hello from testcmd"));
    }

    #[test]
    fn test_repl_handles_nonexistent_executable() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();
        std::env::set_var("PATH", temp_path);

        let input = Cursor::new("nonexistent\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("nonexistent: not found"));
    }

    #[test]
    fn test_repl_prioritizes_builtin_over_executable() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();
        std::env::set_var("PATH", temp_path);

        // Create a mock executable named 'echo'
        let script_path = temp_dir.path().join("echo");
        fs::write(&script_path, "#!/bin/sh\necho 'Mock echo'\n")
            .expect("Failed to create mock echo");
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))
            .expect("Failed to set executable permissions");

        let input = Cursor::new("echo hello\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("hello"));
        assert!(!output_str.contains("Mock echo"));
    }
}

#[cfg(test)]
mod pwd_command_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_repl_handles_pwd_command() {
        let input = Cursor::new("pwd\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains(&env::current_dir().unwrap().to_string_lossy().to_string()));
    }
}

#[cfg(test)]
mod cd_command_tests {
    use super::*;
    // use std::env;
    use std::fs;

    #[test]
    fn test_repl_handles_cd_absolute_path() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();

        let input = Cursor::new(format!("cd {}\npwd\nexit\n", temp_path));
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains(temp_path));
    }

    #[test]
    fn test_repl_handles_cd_relative_path() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).expect("Failed to create subdir");
        let subdir_path = subdir.to_str().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();

        // Start in temp_dir
        env::set_current_dir(temp_path).expect("Failed to set current dir");

        let input = Cursor::new("cd subdir\npwd\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains(subdir_path));
    }

    #[test]
    fn test_repl_handles_cd_no_args() {
        let input = Cursor::new("cd\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("cd: no arguments provided"));
    }

    #[test]
    fn test_repl_handles_cd_too_many_args() {
        let input = Cursor::new("cd /tmp /home\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("cd: too many arguments"));
    }

    #[test]
    fn test_repl_handles_cd_invalid_path() {
        let input = Cursor::new("cd /nonexistent\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains("cd: /nonexistent: No such file or directory"));
    }

    #[test]
    fn test_repl_handles_cd_home_directory() {
        let home_dir = env::var("HOME").expect("HOME not set");
        let input = Cursor::new("cd ~\npwd\nexit\n");
        let mut output = Vec::new();

        let result = run_repl(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("$ "));
        assert!(output_str.contains(&home_dir));
    }
}
