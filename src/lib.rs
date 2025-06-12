use std::io::{self, BufRead, Write};

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

        const BUILT_INS: [&str; 4] = ["exit", "version", "echo", "type"];

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

                // debugging output to show the executable being checked
                writeln!(writer, "Checking type of: {}", executable)?;

                // debug arguments obtained from the command
                writeln!(writer, "Arguments: {:?}", args)?;

                // check if the executable provided is a shell builtin
                if BUILT_INS.contains(&executable) {
                    writeln!(writer, "{} is a shell builtin", executable)?;
                    continue;
                }

                // if the executable is not a shell builtin, check if it exists in the PATH
                let path = std::env::var("PATH").unwrap_or_default();

                // debugging output to show the PATH being checked
                writeln!(writer, "Checking PATH: {}", path)?;

                // Split the PATH by the system's path separator
                // let separator = std::path::MAIN_SEPARATOR;
                let separator = ":";

                // debug the separator used for splitting the PATH
                println!("Using separator: {}", separator);

                // set a flag to indicate if the executable was found
                let mut found = false;

                // Iterate over each directory in the PATH
                for dir in path.split(separator) {
                    // Construct the full path to the executable
                    let full_path = std::path::Path::new(&dir).join(executable);

                    // Check if the file exists and is a regular file
                    println!("Checking: {}", full_path.display());

                    if full_path.exists() {
                        writeln!(writer, "{} is {}", executable, full_path.display())?;
                        found = true;
                        break;
                    }
                }

                if !found {
                    writeln!(writer, "type: {}: not found", executable)?;
                }
            }
            _ => {
                writeln!(writer, "{}: command not found", input)?;
            }
        }
    }
}

#[cfg(test)] // only `cargo test` uses this part of the file
mod tests;
