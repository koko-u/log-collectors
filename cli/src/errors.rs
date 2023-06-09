#[derive(Debug, derive_more::Display)]
#[display(fmt = "CLI Error")]
pub struct CliError;

impl error_stack::Context for CliError {}
