use std::rc::Rc;

use crate::{
    entity::*, 
    world::entity_collection::EntityCollection
};

impl EntityCollection {
    pub fn closest_to(&self, position: &Position) -> Option<Rc<dyn Entity>> {
        self
            .entities()
            .into_iter()
            .min_by_key(|entity| {
                entity.position().distance_to(position).square_dist()
            })
    }
}