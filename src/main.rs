use ipcalc::cli::{describe, ErrorKind};

fn main() {
    let mut cli = describe::CLI::new(std::io::stdout());
    let args = std::env::args().skip(1).collect();
    match cli.execute(args) {
        Ok(()) => {}
        Err(ErrorKind::InvalidInput(reason)) => {
            eprintln!("{}", reason);
            std::process::exit(1);
        }
    }
}
