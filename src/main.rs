use simple_shell::run_repl;
use std::io::{self, stdin, stdout};

fn main() -> io::Result<()> {
    let stdin = stdin().lock();
    let stdout = stdout();
    run_repl(stdin, stdout)
}
