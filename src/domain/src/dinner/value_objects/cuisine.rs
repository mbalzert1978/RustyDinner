use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Cuisine(String);

impl std::str::FromStr for Cuisine {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
