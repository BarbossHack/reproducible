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

    let version_code = if let Some(version_code) = args.version_code {
        // specific version
        eprintln!("version_code: {}", version_code);
        version_code
    } else {
        // latest version
        let details = api.details(&args.package).await.unwrap().unwrap();
        let latest = details.item.unwrap().details.unwrap().app_details.unwrap();
        let version_string = latest.version_string.unwrap();
        let version_code = latest.version_code.unwrap();
        eprintln!("version_string: {}", version_string);
        eprintln!("version_code: {}", version_code);
        std::fs::create_dir(&args.output).unwrap_or_default();

        let mut version_file = args.output.clone();
        version_file.push("VERSION");
        std::fs::write(version_file, &version_string).unwrap();

        let mut version_file = args.output.clone();
        version_file.push("VERSION_CODE");
        std::fs::write(version_file, version_code.to_string()).unwrap();

        latest.version_code.unwrap()
    };

    if args.download {
        std::fs::create_dir(&args.output).unwrap_or_default();
        api.download(
            &args.package,
            Some(version_code),
            true,
            false,
            true,
            &args.output,
            None,
        )
        .await
        .unwrap();
    }
}
