use clap::Parser;
use ipcalc::cli::{describe, split, ErrorKind};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// subdivide the CIDR to create networks of MASK size and displays all resulting networks
    #[clap(long, short = 's', value_parser, value_name = "MASK")]
    split: Option<String>,

    #[clap(value_parser)]
    cidr: String,
}

fn main() {
    let args = Args::parse();

    if let Some(raw_mask) = args.split {
        let mut cli = split::CLI::new(std::io::stdout());
        exec(cli.execute(args.cidr, raw_mask))
    } else {
        let mut cli = describe::CLI::new(std::io::stdout());
        exec(cli.execute(args.cidr))
    }
}

fn exec(rst: Result<(), ErrorKind>) {
    match rst {
        Ok(()) => {}
        Err(ErrorKind::InvalidInput(reason)) => {
            eprintln!("{}", reason);
            std::process::exit(1);
        }
    }
}
