use thiserror::Error;

#[derive(Debug, Error)]
pub enum CargoPublishWSError {
    #[error("Implementation in progress")]
    TodoError,
}
