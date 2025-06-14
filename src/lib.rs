use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;

mod helpers;

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

        let (command, args) = match helpers::parse_input(input) {
            Ok((cmd, args)) => (cmd, args),
            Err(e) => {
                writeln!(writer, "{}", e)?;
                continue;
            }
        };

        if command.is_empty() {
            continue;
        }

        const BUILT_INS: [&str; 6] = ["exit", "version", "echo", "type", "pwd", "cd"];

        match command.as_str() {
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
                if args.is_empty() {
                    writeln!(writer, "type: no arguments provided")?;
                    continue;
                }
                if args.len() != 1 {
                    writeln!(writer, "type: expected exactly one argument")?;
                    continue;
                }
                let executable = &args[0];
                if BUILT_INS.contains(&executable.as_str()) {
                    writeln!(writer, "{} is a shell builtin", executable)?;
                    continue;
                }

                let path = env::var("PATH").unwrap_or_default();
                let separator = ":";

                let mut found = false;
                for dir in path.split(separator) {
                    let full_path = Path::new(dir).join(executable);
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
            "pwd" => match env::current_dir() {
                Ok(path) => writeln!(writer, "{}", path.display())?,
                Err(e) => writeln!(writer, "pwd: {}", e)?,
            },
            "cd" => {
                if args.is_empty() {
                    let home = env::var("HOME")
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "HOME not set"))?;
                    match env::set_current_dir(&home) {
                        Ok(()) => {}
                        Err(e) => writeln!(writer, "cd: invalid home directory: {}", e)?,
                    }
                    continue;
                }
                if args.len() > 1 {
                    writeln!(writer, "cd: too many arguments")?;
                    continue;
                }
                let path_str = if args[0] == "~" {
                    env::var("HOME")
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "HOME not set"))?
                } else {
                    args[0].clone()
                };
                let path = Path::new(&path_str);
                match env::set_current_dir(path) {
                    Ok(()) => {}
                    Err(e) => writeln!(writer, "cd: {}: {}", path_str, e)?,
                }
            }
            _ => {
                let path = env::var("PATH").unwrap_or_default();
                let separator = ":";

                let mut found = false;
                let mut full_path = Path::new(&command).to_path_buf();
                for dir in path.split(separator) {
                    let candidate = Path::new(dir).join(&command);
                    if candidate.exists() {
                        full_path = candidate;
                        found = true;
                        break;
                    }
                }

                if found {
                    let output = Command::new(&full_path)
                        .args(&args)
                        .output()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    if !output.stdout.is_empty() {
                        writer.write_all(&output.stdout)?;
                        if !output.stdout.ends_with(b"\n") {
                            writeln!(writer)?;
                        }
                    }

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

#[cfg(test)]
mod tests;
