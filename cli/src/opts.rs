#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Opt {
    /// log-collectors server url
    #[arg(short, long, value_name = "URL")]
    pub server: String,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// get logs
    Get {
        /// log format [csv, json]
        #[arg(short, long, value_name = "FORMAT", value_enum, default_value_t = LogFormat::Json)]
        format: LogFormat,
    },
    /// post logs, taking input from stdin
    Post,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum LogFormat {
    /// csv format
    Csv,
    /// json format
    Json,
}
