use clap::Parser;
use ipcalc::cli::{describe, ErrorKind};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[clap(value_parser)]
    cidr: String,
}

fn main() {
    let args = Args::parse();

    let mut cli = describe::CLI::new(std::io::stdout());
    match cli.execute(args.cidr) {
        Ok(()) => {}
        Err(ErrorKind::InvalidInput(reason)) => {
            eprintln!("{}", reason);
            std::process::exit(1);
        }
    }
}
