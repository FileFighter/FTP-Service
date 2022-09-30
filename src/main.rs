use std::sync::Arc;
use tracing::{info, metadata::LevelFilter, Level};
use tracing_subscriber::{filter::Targets, fmt::time::SystemTime, prelude::*};
use unftp_filefighter::{FileFighter, FileFighterAuthenticator};

#[tokio::main]
pub async fn main() {
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
                .with_target("filefighter_api", LevelFilter::DEBUG)
                .with_target("ftp_fighter", LevelFilter::DEBUG)
                .with_target("unftp_filefighter", LevelFilter::DEBUG)
                // Disable unauth logs
                .with_target("libunftp", LevelFilter::OFF)
                .with_default(Level::INFO),
        )
        .init();

    info!("Started FTP Server");

    libunftp::Server::with_authenticator(
        Box::new(move || FileFighter::new()),
        Arc::new(FileFighterAuthenticator::new()),
    )
    .greeting("FileFighter FTP server")
    .passive_ports(5000..5100)
    .listen("127.0.0.1:2121")
    .await
    .unwrap();
}
