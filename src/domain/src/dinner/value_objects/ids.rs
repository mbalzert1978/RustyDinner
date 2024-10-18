use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DinnerId(String);

impl std::str::FromStr for DinnerId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservationId(String);

impl std::str::FromStr for ReservationId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
