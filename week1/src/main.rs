/*Command-line interface for Marco Polo */

use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Yuzhou Zhao",
    after_help = "Command-line interface for Marco Polo"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yuzhou Zhao")]
    MarcoPolo {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    if let Some(Commands::MarcoPolo { input }) = args.command {
        let result = week1::marco_polo(&input);
        println!("{}", result);
    } else {
        println!("No command was used")
    }
}
