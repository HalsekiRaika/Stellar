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
    },
    #[error("")]
    Unavailable {
        
    },
    #[error("An error occurred while processing the value.")]
    Processing,
    #[error("Wrong data was used. {reason}")]
    Invalid {
        reason: &'static str
    },
    #[error("An error occurred in an external library:{crate_name} unrelated to the kernel.")]
    External {
        crate_name: &'static str,
    }
}
