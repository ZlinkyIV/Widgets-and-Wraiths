use std::rc::Rc;

use crate::entity::*;
use crate::world::world_context::WorldContext;

impl<'a> WorldContext {
    pub fn closest_to(&self, position: &Position) -> Option<Rc<Entity>> {
        self
            .entities()
            .min_by_key(|entity| {
                entity.position().distance_to(position).square_dist()
            })
    }
}