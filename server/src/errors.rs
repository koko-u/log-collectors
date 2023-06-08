#[derive(Debug, derive_more::Display)]
#[display(fmt = "Application Error")]
pub struct AppError;

impl error_stack::Context for AppError {}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]

pub enum AppResponseError {
    #[display(fmt = "Multipart Error {0}", _0)]
    MultiPartError(#[error(source)] actix_multipart::MultipartError),
    #[display(fmt = "Other Response Error")]
    Other,
}

impl actix_web::ResponseError for AppResponseError {}

impl<C> From<error_stack::Report<C>> for AppResponseError
where
    C: error_stack::Context,
{
    fn from(report: error_stack::Report<C>) -> Self {
        log::error!("{report:?}");
        Self::Other
    }
}
