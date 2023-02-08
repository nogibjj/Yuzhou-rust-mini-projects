/*Command-line interface for Integer To Roman conversion */

use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Yuzhou Zhao",
    after_help = "Command-line interface for Integer To Roman conversion"
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

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    integer: String,
}

#[derive(Serialize)]
struct Response {
    result: String,
}
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let integer_str = event.payload.integer;
    let num: i32 = integer_str.parse().unwrap();
    let res = week3::int_to_roman(num);

    // Prepare the response
    let resp = Response {
        result: format!("{}", res),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
