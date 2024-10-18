#[derive(Debug, PartialEq)]
pub enum DinnerStatus {
    Cancelled,
    Completed,
    Open,
    Ordered,
    Preparing,
    Served,
}
