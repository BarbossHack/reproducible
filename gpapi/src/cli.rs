use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "gpapi")]
#[command(about = "Google Play API Client", long_about = None)]
pub struct Args {
    #[arg(short, long, env)]
    pub email: String,

    #[arg(short, long, env)]
    pub aas_token: String,

    #[arg(short, long)]
    pub package: String,

    #[arg(short, long)]
    pub version_code: Option<i64>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
