use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Yuzhou Zhao",
    about = "Compare serial and variable thread exectution of QuickSort",
    after_help = "Example: cargo run quick-sort --size 1000000"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    QuickSort {
        #[clap(short, long, default_value = "1000000")]
        size: u32,
    },
}

use std::time::Instant;
use rand::{thread_rng, Rng, seq::SliceRandom};

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::QuickSort {size}) =>{
            println!("Sorting vector of size: {:?}", size);
            let mut vec: Vec<u32> = (0..size).collect();
            let mut rng = rand::thread_rng();
            vec.shuffle(&mut rng);
            
            let start = Instant::now();
            // create deep copy of vec
            let mut vec_parallel = vec.clone();
            week8::quick_sort_parallel(&mut vec_parallel);
            let dur = Instant::now() - start;
            let nanos_parallel = u64::from(dur.subsec_nanos()) + dur.as_secs() * 1_000_000_000u64;
            println!("Nanosecond elapsed: {:?}", nanos_parallel);
            println!("is sorted: {:?}", week8::do_vecs_match(&vec_parallel, &(0..size).collect()));

            let start = Instant::now();
            // create deep copy of vec
            let mut vec_serial = vec.clone();
            week8::quick_sort_serial(&mut vec_serial);
            let dur = Instant::now() - start;
            let nanos_serial = u64::from(dur.subsec_nanos()) + dur.as_secs() * 1_000_000_000u64;
            println!("Nanosecond elapsed: {:?}", nanos_serial);
            println!("is sorted: {:?}", week8::do_vecs_match(&vec_serial, &(0..size).collect()));
            
            println!("speedup: {:?}", nanos_serial as f64 / nanos_parallel as f64);
        },
        None => {
            println!("No command given");
        }
    }
}