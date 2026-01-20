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
    let latest = details.item.unwrap().details.unwrap().app_details.unwrap();
    let version_string = latest.version_string.unwrap();
    let version_code = latest.version_code.unwrap();

    if let Some(output) = &args.output {
        std::fs::create_dir(output).unwrap_or_default();
        api.download(
            &args.package,
            Some(version_code),
            true,
            false,
            true,
            &output,
            None,
        )
        .await
        .unwrap();
        let mut version_file = output.clone();
        version_file.push("VERSION");
        std::fs::write(version_file, &version_string).unwrap();
    }

    println!("{}", version_string);
}
