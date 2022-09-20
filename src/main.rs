use std::sync::Arc;
use unftp_auth_filefighter::FileFighterAuthenticator;
use unftp_sbe_filefighter::ServerExt;

#[tokio::main]
// IDEA: handle args
pub async fn main() {
    let server = libunftp::Server::connect_to_filefighter()
        .greeting("Welcome to my FileFighter FTP server")
        .authenticator(Arc::new(FileFighterAuthenticator::new()))
        .passive_ports(50000..65535);

    server.listen("127.0.0.1:2121").await.unwrap()
}
