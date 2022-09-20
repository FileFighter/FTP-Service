use crate::backend::FileFighter;
use libunftp::auth::DefaultUser;
use libunftp::Server;

pub trait ServerExt {
    fn connect_to_filefighter() -> Server<FileFighter, DefaultUser> {
        libunftp::Server::new(Box::new(move || FileFighter::new()))
    }
}

impl ServerExt for Server<FileFighter, DefaultUser> {}
