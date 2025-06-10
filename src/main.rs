use simple_shell::run_repl;
use std::io::{self, stdin, stdout};
use std::process;

fn main() -> io::Result<()> {
    let stdin = stdin().lock();
    let stdout = stdout();
    let exit_code = run_repl(stdin, stdout)?;
    process::exit(exit_code);
}
