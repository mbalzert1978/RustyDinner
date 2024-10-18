use super::Dinner;
use super::DinnerId;
use crate::prelude::*;

trait DinnerRepository {
    async fn get_dinner_by_id(&self, id: DinnerId) -> Result<Dinner>;
}
