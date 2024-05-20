use std::rc::Rc;

use crate::{
    entity::*, 
    world::entity_collection::EntityCollection
};

impl<'a> EntityCollection {
    pub fn closest_to(&self, position: &Position) -> Option<Rc<Entity>> {
        self
            .entities()
            .into_iter()
            .map(|entity| entity)
            .min_by_key(|entity| {
                entity.position().distance_to(position).square_dist()
            })
    }
}