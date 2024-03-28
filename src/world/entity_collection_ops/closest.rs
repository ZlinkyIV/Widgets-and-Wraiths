use std::rc::Rc;

use crate::{
    entity::*, 
    world::entity_collection::EntityCollection
};

impl<'a> EntityCollection<'a> {
    pub fn closest_to(&self, position: &Position) -> Option<Rc<dyn Entity<'a> + 'a>> {
        self
            .entities()
            .into_iter()
            .map(|(_, entity)| entity)
            .min_by_key(|entity| {
                entity.position().distance_to(position).square_dist()
            })
    }
}