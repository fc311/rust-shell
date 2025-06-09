use std::io::{self, BufRead, Write};

pub fn run_repl<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    loop {
        write!(writer, "$ ")?;
        writer.flush()?;

        let mut input = String::new();
        reader.read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }
        if input == "exit" {
            break;
        }

        writeln!(writer, "{}: command not found", input)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
