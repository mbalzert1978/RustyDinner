use crate::dinner::DinnerId;
use crate::host::HostId;

#[derive(Debug)]
pub struct GuestRating {
    rating_id: RatingId,
    dinner_id: DinnerId,
    host_id: HostId,
    user_id: UserId,
    rating: Rating,
    comment: Comment,
}
