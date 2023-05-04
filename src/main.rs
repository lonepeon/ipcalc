use clap::{Parser, Subcommand};
use ipcalc::cli::{compare, describe, split, ErrorKind};

#[derive(Subcommand, Debug)]
enum CLICommand {
    #[clap(about=DESCRIBE_HELP)]
    Describe {
        #[clap(help=DESCRIBE_CIDR_HELP)]
        cidr: String,
        #[clap(help=DESCRIBE_NO_BINARY_HELP, long)]
        no_binary: bool,
    },
    #[clap(about=SPLIT_HELP, long_about=SPLIT_LONG_HELP)]
    Split {
        #[clap(help=SPLIT_CIDR_HELP)]
        cidr: String,
        #[clap(help=SPLIT_NEW_MASK)]
        new_mask: String,
        #[clap(help=SPLIT_NO_BINARY_HELP, long)]
        no_binary: bool,
    },
    #[clap(about=COMPARE_HELP, long_about=COMPARE_LONG_HELP)]
    Compare {
        #[clap(help=COMPARE_CIDR_HELP)]
        cidr: String,
        #[clap(help=COMPARE_CIDR_OTHER_HELP)]
        other: String,
    },
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: CLICommand,
}

fn main() {
    if let Err(ErrorKind::InvalidInput(err)) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), ErrorKind> {
    let cli = Cli::parse();

    match cli.command {
        CLICommand::Describe { cidr, no_binary } => {
            let mut cli = describe::CLI::new(std::io::stdout());
            cli.with_binary = !no_binary;
            cli.execute(cidr)
        }
        CLICommand::Split {
            cidr,
            new_mask,
            no_binary,
        } => {
            let mut cli = split::CLI::new(std::io::stdout());
            cli.with_binary = !no_binary;
            cli.execute(cidr, new_mask)
        }
        CLICommand::Compare { cidr, other } => {
            let mut cli = compare::CLI::new(std::io::stdout());
            cli.execute(cidr, other)
        }
    }
}

static DESCRIBE_HELP: &str = "Display host and network related information about the IPv4 CIDR";
static DESCRIBE_CIDR_HELP: &str = "Any valid host or network IPv4 CIDR";
static DESCRIBE_NO_BINARY_HELP: &str = "Hide the binary representation";

static SPLIT_HELP: &str = "Subdivide the CIDR in smaller networks and display them";
static SPLIT_LONG_HELP: &str = "Subdivide the CIDR in smaller networks and display them

If the CIDR is a network address: display all available sub-networks
If the CIDR is a host address: display the new network in which the IP belongs
";
static SPLIT_CIDR_HELP: &str = DESCRIBE_CIDR_HELP;
static SPLIT_NEW_MASK: &str = "New prefix length to apply to the CIDR
";
static SPLIT_NO_BINARY_HELP: &str = DESCRIBE_NO_BINARY_HELP;

static COMPARE_HELP: &str = "Compare two CIDRs and display the relationship between them";
static COMPARE_LONG_HELP: &str =
    "Compare two CIDRs and display the relationship between the first and second CIDR:
same network, different network, subset or superset";
static COMPARE_CIDR_HELP: &str = "Any valid host or network IPv4 CIDR.
If an host CIDR is given, its related network will be used.";
static COMPARE_CIDR_OTHER_HELP: &str = COMPARE_CIDR_HELP;
