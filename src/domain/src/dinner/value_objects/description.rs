use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Description(String);

impl std::str::FromStr for Description {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
