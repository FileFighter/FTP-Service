use clap::Parser;
use cli::Args;
use dotenvy::dotenv;
use filefighter_api::ffs_api::ApiConfig;
use libunftp::ServerError;
use std::{ops::Range, sync::Arc};
use tracing::{debug, info, metadata::LevelFilter, Level};
use tracing_subscriber::{filter::Targets, fmt::time::SystemTime, prelude::*};
use unftp_filefighter::{FileFighter, FileFighterAuthenticator};

mod cli;

pub fn setup_logging(args: &Args) {
    color_eyre::install().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(SystemTime::default()) // i think this is the default one
                .with_ansi(true),
        )
        .with(
            Targets::new()
                // Own crates debug
                .with_target("filefighter_api", args.log_level)
                .with_target("ftp_fighter", args.log_level)
                .with_target("unftp_filefighter", args.log_level)
                // Disable unauth logs
                .with_target("libunftp", LevelFilter::OFF)
                .with_default(Level::INFO),
        )
        .init();
}

pub fn parse_cli_args() -> Args {
    // read from env
    dotenv().ok();

    let mut args = Args::parse();
    // add missing uri components
    args.backend_url.push_str("/api");
    args.filehandler_url.push_str("/data");
    args
}

pub async fn start_ftp_service(args: Args) -> Result<(), ServerError> {
    let api_config: ApiConfig = args.clone().into();
    let api_config_clone = api_config.clone();

    info!("Starting FTP Server...");
    debug!("Config: {:#?}", args);

    libunftp::Server::with_authenticator(
        Box::new(move || FileFighter {
            api_config: api_config.clone(),
        }),
        Arc::new(FileFighterAuthenticator {
            api_config: api_config_clone,
        }),
    )
    .greeting("FileFighter FTP server")
    .passive_ports(Range {
        start: args.passive_start_port,
        end: args.passive_end_port,
    })
    .listen(format!("{}:{}", args.hostname, args.port))
    .await
}
