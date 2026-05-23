use rusqlite::Error;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum PlanifyError {
    #[error("Database not found at the expected location: {searched:?}. {hint}")]
    DbNotFound {
        searched: Vec<PathBuf>,
        hint: String,
    },

    #[error("Error in the database {0}")]
    DbError(#[from] Error),

    #[error("Database locked {hint}")]
    DbLocked { hint: String },
}
