use clap::Parser;
use cli::errors::CliError;
use cli::opts::Command;
use cli::opts::Opt;
use cli::requests::get_logs;
use cli::requests::post_logs;
use env_logger::Env;
use error_stack::IntoReport;
use error_stack::ResultExt;

fn main() -> error_stack::Result<(), CliError> {
    dotenv::dotenv().into_report().change_context(CliError)?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opt::parse();

    match opt.command {
        Command::Get { format } => get_logs(&opt.server, format)?,
        Command::Post => post_logs(&opt.server)?,
    }
    Ok(())
}
