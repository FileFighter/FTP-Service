use clap::Parser;
use filefighter_api::ffs_api::ApiConfig;
use tracing::metadata::LevelFilter;

/// FileFighter FTP-Service
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Used FTP-Server hostname
    // IDEA: url validation?
    #[arg(short = 'n', long, env = "FTP_SERVICE_HOSTNAME", default_value_t = String::from("0.0.0.0"))]
    pub hostname: String,

    /// Used FTP-Server port
    #[arg(short, long, env = "FTP_SERVICE_PORT", value_parser = clap::value_parser!(u16).range(1..), default_value_t = 2121)]
    pub port: u16,

    /// Passive port range start to use for file transfers
    #[arg(short = 's', long, value_parser = clap::value_parser!(u16).range(1..), env = "FTP_SERVICE_PASSIVE_START", default_value_t = 10000)]
    pub passive_start_port: u16,

    /// Passive port range end to use for file transfers
    #[arg(short = 'e', long, value_parser = clap::value_parser!(u16).range(1..), env = "FTP_SERVICE_PASSIVE_END", default_value_t = 10010)]
    pub passive_end_port: u16,

    /// Base url of the FileSystemService
    #[arg(short, long, env = "FTP_SERVICE_LOG_LEVEL", default_value_t = LevelFilter::INFO)]
    pub log_level: LevelFilter,

    /// Base url of the FileSystemService (without trailing slash) eg. http://localhost:8080
    #[arg(short, long, env = "FTP_SERVICE_BACKEND_URL")]
    pub backend_url: String,

    /// Base url of the FileHandlerService (without trailing slash) eg. http://localhost:5000
    #[arg(short, long, env = "FTP_SERVICE_FILEHANDLER_URL")]
    pub filehandler_url: String,
}

/// Implement conversion between config and args
impl From<Args> for ApiConfig {
    fn from(args: Args) -> Self {
        let mut backend_url = args.backend_url;
        backend_url.push_str("/api");

        let mut filehandler_url = args.filehandler_url;
        filehandler_url.push_str("/data");

        Self {
            fss_base_url: backend_url,
            fhs_base_url: filehandler_url,
        }
    }
}
