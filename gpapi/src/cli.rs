use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "gpapi")]
#[command(about = "Google Play API Client", long_about = None)]
pub struct Args {
    #[arg(long, env)]
    pub email: String,

    #[arg(long, env)]
    pub aas_token: String,

    #[arg(long, env)]
    pub oauth_token: Option<String>,

    #[arg(long)]
    pub package: String,

    #[arg(long)]
    pub version_code: Option<i64>,

    #[arg(long)]
    pub output: PathBuf,

    #[arg(long, default_value = "false")]
    pub download: bool,
}
