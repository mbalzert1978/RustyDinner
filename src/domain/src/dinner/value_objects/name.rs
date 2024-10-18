use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);

impl std::str::FromStr for Name {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
