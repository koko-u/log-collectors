use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, derive_more::Display,
)]
#[display(fmt = "CSV Response [{} bytes]", _0)]
pub struct CsvResponse(pub u64);
