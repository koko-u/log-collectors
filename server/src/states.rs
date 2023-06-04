use error_stack::IntoReport;
use error_stack::ResultExt;
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Clone, derive_more::Deref, derive_more::From)]
pub struct DbState(PgPool);

impl DbState {
    pub async fn new(database_url: &str) -> error_stack::Result<Self, AppError> {
        let pool = PgPool::connect(database_url)
            .await
            .into_report()
            .change_context(AppError)?;
        Ok(Self(pool))
    }
}
