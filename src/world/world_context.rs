use std::rc::Rc;

use crate::entity::Entity;

// use super::entity_collection_ops::filter::Filter;

#[derive(Clone)]
pub struct WorldContext {
    pub entities: Rc<Vec<Rc<Entity>>>,
}

impl WorldContext {
    pub fn entities(&self) -> impl Iterator<Item = Rc<Entity>> + '_ {
        self.entities.iter().map(|entity| entity.clone())
    }

    // pub fn apply_filters(self, filters: &[&dyn Filter]) -> WorldContext {

    // }
}