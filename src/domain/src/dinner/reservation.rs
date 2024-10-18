use super::value_objects::ids::ReservationId;

#[derive(Debug)]
pub struct Reservation {
    id: ReservationId,
    guest_id: GuestId,
    bil_id: BilId,
}
