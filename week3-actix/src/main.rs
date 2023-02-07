/*Command-line interface for Simple Calculator */

use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Yuzhou Zhao",
    after_help = "Command-line interface for Simple Calculator"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yuzhou Zhao")]
    Calculator {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    if let Some(Commands::Calculator { input }) = args.command {
        let result = week2::calculate(input);
        println!("{}", result);
    } else {
        println!("No command was used")
    }
}
