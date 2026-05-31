mod cli;

use clap::Parser;
use gpapi::Gpapi;

/**
 * Follow https://github.com/EFForg/apkeep/blob/master/USAGE-google-play.md steps.
 */

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = cli::Args::parse();

    let mut api = Gpapi::new("px_9a", &args.email);
    api.request_aas_token(args.oauth_token.unwrap())
        .await
        .unwrap();
    println!("{:?}", api.get_aas_token());
}
