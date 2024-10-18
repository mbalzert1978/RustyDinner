pub use value_objects::host_id::HostId;

pub mod value_objects;

#[derive(Debug)]
pub struct Host(HostId);
