#[derive(Debug, derive_more::Display)]
#[display(fmt = "Application Error")]
pub struct AppError;

impl error_stack::Context for AppError {}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(fmt = "Response Error")]
pub struct AppResponseError;

impl actix_web::ResponseError for AppResponseError {}

impl<C> From<error_stack::Report<C>> for AppResponseError
where
    C: error_stack::Context,
{
    fn from(report: error_stack::Report<C>) -> Self {
        log::error!("{report:?}");
        Self
    }
}
