use clap::Parser;
use filefighter_api::ffs_api::ApiConfig;
use tracing::metadata::LevelFilter;

/// FileFighter FTP-Service
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Url to listen on eg. 0.0.0.0:2121
    #[arg(short, long, env = "FTP_SERVICE_URL")]
    pub url: String,

    /// Passive port range start to use for file transfers
    #[arg(short = 's', long, value_parser = clap::value_parser!(u16).range(1..), env = "FTP_SERVICE_PASSIVE_START", default_value_t = 10000)]
    pub passive_start_port: u16,

    /// Passive port range end to use for file transfers
    #[arg(short = 'e', long, value_parser = clap::value_parser!(u16).range(1..), env = "FTP_SERVICE_PASSIVE_END", default_value_t = 10010)]
    pub passive_end_port: u16,

    /// Base url of the FileSystemService
    #[arg(short, long, env = "FTP_SERVICE_LOG_LEVEL", default_value_t = LevelFilter::DEBUG)]
    pub log_level: LevelFilter,

    /// Base url of the FileSystemService eg. http://localhost:8080
    #[arg(short, long, env = "FTP_SERVICE_BACKEND_URL")]
    pub backend_url: String,

    /// Base url of the FileHandlerService eg. http://localhost:5000
    #[arg(short, long, env = "FTP_SERVICE_FILEHANDLER_URL")]
    pub filehandler_url: String,
}

/// Implement conversion between config and args
impl From<Args> for ApiConfig {
    fn from(args: Args) -> Self {
        Self {
            fss_base_url: args.backend_url,
            fhs_base_url: args.filehandler_url,
        }
    }
}
