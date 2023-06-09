use std::io;

use api::requests::logs::NewLog;
use error_stack::IntoReport;
use error_stack::ResultExt;

use crate::errors::CliError;
use crate::opts::LogFormat;

pub fn get_logs(server: &str, format: LogFormat) -> error_stack::Result<(), CliError> {
    let client = reqwest::blocking::Client::default();
    let uri = match format {
        LogFormat::Json => format!("{server}/logs"),
        LogFormat::Csv => format!("{server}/csv"),
    };
    let mut response = client
        .get(uri)
        .send()
        .into_report()
        .change_context(CliError)?;

    let mut stdout = io::stdout().lock();
    response
        .copy_to(&mut stdout)
        .into_report()
        .change_context(CliError)?;

    Ok(())
}

pub fn post_logs(server: &str) -> error_stack::Result<(), CliError> {
    let stdin = io::stdin().lock();
    let new_logs = csv::ReaderBuilder::default()
        .has_headers(false)
        .from_reader(stdin)
        .into_deserialize::<NewLog>();

    let client = reqwest::blocking::Client::default();

    for log in new_logs {
        match log {
            Ok(log) => client
                .post(format!("{server}/logs"))
                .json(&log)
                .send()
                .into_report()
                .change_context(CliError)?,
            Err(e) => {
                log::error!("{e:?}");
                continue;
            }
        };
    }

    Ok(())
}
