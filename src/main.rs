use clap::{Parser, Subcommand};
use ipcalc::cli::{compare, describe, split, ErrorKind};

#[derive(Subcommand, Debug)]
enum CLICommand {
    /// Details all the information about the CIDR
    Describe {
        /// Any valid host or network IPv4 CIDR
        cidr: String,
        /// Hide the binary representation when display the CIDR information
        #[clap(long)]
        no_binary: bool,
    },
    /// Subdivide the CIDR to create networks of MASK size and displays all resulting networks
    Split {
        /// Any valid host or network IPv4 CIDR
        cidr: String,
        /// New prefix length to apply to the IPv4 CIDR.
        /// If the CIDR is a host address, displays the network in which it would be with the new mask.
        /// If the CIDR is a network address, displays all sub-networks using the new mask
        new_mask: String,
        /// Hide the binary representation when display the CIDR information
        #[clap(long)]
        no_binary: bool,
    },
    /// Compare two CIDR and displays the relationship between each other
    Compare {
        /// Any valid host or network IPv4 CIDR. If an host CIDR is given, its related network will be used.
        cidr: String,
        /// Any valid host or network IPv4 CIDR. If an host CIDR is given, its related network will be used.
        other: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: CLICommand,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CLICommand::Describe { cidr, no_binary } => {
            let mut cli = describe::CLI::new(std::io::stdout());
            cli.with_binary = !no_binary;
            exec(cli.execute(cidr))
        }
        CLICommand::Split {
            cidr,
            new_mask,
            no_binary,
        } => {
            let mut cli = split::CLI::new(std::io::stdout());
            cli.with_binary = !no_binary;
            exec(cli.execute(cidr, new_mask))
        }
        CLICommand::Compare { cidr, other } => {
            let mut cli = compare::CLI::new(std::io::stdout());
            exec(cli.execute(cidr, other))
        }
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
