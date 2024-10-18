use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Location(String);

impl std::str::FromStr for Location {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
