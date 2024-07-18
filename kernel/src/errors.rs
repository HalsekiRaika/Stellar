#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("entity-specific validation rules are violated. entity={entity}, reason={reason}")]
    Validation {
        entity: &'static str,
        reason: &'static str,
    },
    #[error("this value already exist. {val}")]
    AlreadyExist {
        val: String
    },
    #[error("this value not found. {val}")]
    NotFound {
        val: String
    }
}