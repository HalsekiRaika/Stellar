#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("required to execute the program did not exist. {require}")]
    Require {
        require: &'static str
    },
    #[error("An exception occurred in kernel.")]
    Kernel,
    #[error("An error occurred in an external library unrelated to the application.")]
    External
}