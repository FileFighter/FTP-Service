#[tokio::main]
pub async fn main() -> Result<(), libunftp::ServerError> {
    ftp_fighter::setup_logging();
    ftp_fighter::parse_cli_args();
    ftp_fighter::start_ftp_service().await
}
