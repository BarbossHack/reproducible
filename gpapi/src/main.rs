mod cli;

use clap::Parser;
use gpapi::Gpapi;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = cli::Args::parse();

    let mut api = Gpapi::new("px_9a", &args.email);
    api.set_aas_token(args.aas_token);
    api.login().await.unwrap();
    let details = api.details(&args.package).await.unwrap().unwrap();
    eprintln!("{:#?}", details);
    let app_details = details.item.unwrap().details.unwrap().app_details.unwrap();
    let version_string = app_details.version_string.unwrap();
    let version_code = app_details.version_code.unwrap();

    if let Some(output) = &args.output {
        api.download(
            &args.package,
            Some(version_code),
            true,
            true,
            true,
            &output,
            None,
        )
        .await
        .unwrap();
    }

    println!("{}", version_string);
}
