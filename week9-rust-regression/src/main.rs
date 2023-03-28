use clap::Parser;
use ndarray::{Array1, Array2};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use rand::Rng;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Yuzhou Zhao",
    about = "A simple linear regression model",
    after_help = "Example: cargo run regression --samples 1000 --learning-rate 0.01 --epochs 1000 --true-slope 2.0 --true-intercept 1.0"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Regression {
        #[clap(short, long, default_value = "1000")]
        samples: usize,
        #[clap(short, long, default_value = "0.1")]
        learning_rate: f64,
        #[clap(short, long, default_value = "1000")]
        epochs: usize,
        #[clap(short = 'm', long, default_value = "2.0")]
        true_slope: f64,
        #[clap(short = 'b', long, default_value = "10.0")]
        true_intercept: f64,
    }, 
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Regression {samples, learning_rate, epochs, true_slope, true_intercept}) =>{
            println!("Running linear regression model");
            week9::run_linear_regression(samples, learning_rate, epochs, true_slope, true_intercept);
        },
        None => {
            println!("No command given");
        }
    }
}

