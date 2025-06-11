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
                if args.is_empty() {
                    writeln!(writer, "type: no arguments provided")?;
                } else {
                    if args.len() == 1 && BUILT_INS.contains(&args[0]) {
                        writeln!(writer, "{} is a shell builtin", args[0])?;
                    } else {
                        writeln!(writer, "{}: not found", args[0])?;
                    }
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
