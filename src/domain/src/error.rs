#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DomainError {
    Generic,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::Generic => write!(f, "An error occurred in the domain"),
        }
    }
}

impl std::error::Error for DomainError {}
