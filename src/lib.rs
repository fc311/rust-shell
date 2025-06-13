use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;

pub fn run_repl<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<i32> {
    loop {
        write!(writer, "$ ")?;
        writer.flush()?;

        let mut input = String::new();
        reader.read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        const BUILT_INS: [&str; 6] = ["exit", "version", "echo", "type", "pwd", "cd"];

        match command {
            "exit" => {
                if args.is_empty() || (args.len() == 1 && args[0] == "0") {
                    return Ok(0);
                }
                writeln!(writer, "{}: command not found", input)?;
            }
            "version" => {
                writeln!(writer, "Simple Shell v0.1.0")?;
            }
            "echo" => {
                if args.is_empty() {
                    writeln!(writer, "echo: no arguments provided")?;
                } else {
                    writeln!(writer, "{}", args.join(" "))?;
                }
            }
            "type" => {
                // if the command is `type`, but has no arguments, print an error
                if args.is_empty() {
                    writeln!(writer, "type: no arguments provided")?;
                    continue;
                }

                // if the number of arguments after `type` is not exactly one, print an error
                if args.len() != 1 {
                    writeln!(writer, "type: expected exactly one argument")?;
                    continue;
                }

                // extract the executable name from the argument provided with `type`
                let executable = args[0];

                // check if the executable provided is a shell builtin
                if BUILT_INS.contains(&executable) {
                    writeln!(writer, "{} is a shell builtin", executable)?;
                    continue;
                }

                // if the executable is not a shell builtin, check if it exists in the PATH
                let path = std::env::var("PATH").unwrap_or_default();

                // Split the PATH by the system's path separator
                // let separator = std::path::MAIN_SEPARATOR;
                let separator = ":";

                // set a flag to indicate if the executable was found
                let mut found = false;

                // Iterate over each directory in the PATH
                for dir in path.split(separator) {
                    // Construct the full path to the executable
                    let full_path = std::path::Path::new(&dir).join(executable);

                    if full_path.exists() {
                        writeln!(writer, "{} is {}", executable, full_path.display())?;
                        found = true;
                        break;
                    }
                }

                if !found {
                    writeln!(writer, "{}: not found", executable)?;
                }
            }
            "pwd" => {
                // Get the current working directory
                match env::current_dir() {
                    Ok(path) => writeln!(writer, "{}", path.display())?,
                    Err(e) => writeln!(writer, "pwd: {}", e)?,
                }
            }
            "cd" => {
                if args.is_empty() {
                    writeln!(writer, "cd: no arguments provided")?;
                    continue;
                }
                if args.len() > 1 {
                    writeln!(writer, "cd: too many arguments")?;
                    continue;
                }
                let path = Path::new(args[0]);
                if path.to_str().unwrap() == "~" {
                    // If the path is "~", change to the home directory
                    match env::home_dir() {
                        Some(home) => match env::set_current_dir(home) {
                            Ok(()) => {}
                            Err(_) => writeln!(writer, "cd: No such file or directory")?,
                        },
                        None => writeln!(writer, "cd: Home directory not found")?,
                    }
                } else if path == Path::new("/") {
                    // If the path is "/", change to the root directory
                    match env::set_current_dir("/") {
                        Ok(()) => {}
                        Err(_) => writeln!(writer, "cd: No such file or directory")?,
                    }
                } else if path.is_absolute() {
                    match env::set_current_dir(path) {
                        Ok(()) => {}
                        Err(_) => {
                            writeln!(writer, "cd: {}: No such file or directory", path.display())?
                        }
                    }
                } else {
                    match env::set_current_dir(path) {
                        Ok(()) => {}
                        Err(_) => writeln!(writer, "cd: invalid path")?,
                    }
                }
            }
            _ => {
                let path = env::var("PATH").unwrap_or_default();
                let separator = ":";

                let mut found = false;
                let mut full_path = Path::new(command).to_path_buf();
                for dir in path.split(separator) {
                    let candidate = Path::new(dir).join(command);
                    if candidate.exists() {
                        full_path = candidate;
                        found = true;
                        break;
                    }
                }

                if found {
                    let output = Command::new(&command) // Changed from full_path to command
                        .current_dir(full_path.parent().unwrap_or_else(|| Path::new("/"))) // Add this line
                        .args(&args)
                        .output()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    // Write stdout if it's not empty
                    if !output.stdout.is_empty() {
                        writer.write_all(&output.stdout)?;
                        // Only write a newline if the output doesn't end with one
                        if !output.stdout.ends_with(b"\n") {
                            writeln!(writer)?;
                        }
                    }

                    // Write stderr if it's not empty
                    if !output.stderr.is_empty() {
                        writer.write_all(&output.stderr)?;
                        if !output.stderr.ends_with(b"\n") {
                            writeln!(writer)?;
                        }
                    }

                    writer.flush()?;
                } else {
                    writeln!(writer, "{}: not found", command)?;
                }
            }
        }
    }
}

#[cfg(test)] // only `cargo test` uses this part of the file
mod tests;
