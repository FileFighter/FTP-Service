use std::sync::Arc;
use tracing::{info, metadata::LevelFilter, Level};
use tracing_subscriber::{
    filter::Targets, fmt::format::FmtSpan, fmt::time::SystemTime, prelude::*,
};
use unftp_auth_filefighter::*;
use unftp_sbe_filefighter::FileFighter;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(SystemTime::default()) // i think this is the default one
                .with_ansi(true)
                .with_span_events(FmtSpan::ENTER),
        )
        .with(
            Targets::new()
                // Own crates debug
                .with_target("filefighter_api", LevelFilter::DEBUG)
                .with_target("ftp_fighter", LevelFilter::DEBUG)
                .with_target("unftp_auth_filefighter", LevelFilter::DEBUG)
                .with_target("unftp_sbe_filefighter", LevelFilter::DEBUG)
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
    .passive_ports(50000..65535)
    .listen("127.0.0.1:2121")
    .await
    .unwrap();
}
