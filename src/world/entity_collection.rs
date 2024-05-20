use std::rc::Rc;

use crate::entity::Entity;

#[derive(Clone)]
pub struct EntityCollection {
    entities: Vec<Rc<Entity>>,
}

impl EntityCollection {
    pub fn new(entities: Vec<Rc<Entity>>) -> Self {
        Self {
            entities: entities,
        }
    }

    pub fn entities(&self) -> impl Iterator {
        self.entities.iter()
    }
}