mod abstractions;
mod enums;
mod reservation;
mod value_objects;

use crate::host::value_objects::host_id::HostId;
use crate::prelude::*;
use crate::AggregateRoot;
use crate::Entity;

use enums::dinner_status::DinnerStatus;
use value_objects::cuisine::Cuisine;
use value_objects::description::Description;
pub use value_objects::ids::DinnerId;
use value_objects::location::Location;
use value_objects::name::Name;

#[derive(Debug)]
pub struct Dinner {
    id: DinnerId,
    host_id: HostId,
    menu_id: MenuId,
    name: Name,
    description: Description,
    location: Location,
    cuisine: Cuisine,
    status: DinnerStatus,
    reservations: Vec<Reservation>,
}

impl AggregateRoot for Dinner {}

impl Dinner {
    fn new(
        id: DinnerId,
        name: Name,
        description: Description,
        location: Location,
        cuisine: Cuisine,
    ) -> Result<Self> {
        todo!()
    }
}

impl Entity<DinnerId> for Dinner {
    fn id(&self) -> &DinnerId {
        &self.id
    }
}

impl PartialEq for Dinner {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
