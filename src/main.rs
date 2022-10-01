#[tokio::main]
pub async fn main() -> Result<(), libunftp::ServerError> {
    let args = ftp_fighter::parse_cli_args();

    ftp_fighter::setup_logging(&args);
    ftp_fighter::start_ftp_service(args).await
}
