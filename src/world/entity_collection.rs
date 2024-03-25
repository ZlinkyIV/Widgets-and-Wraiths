use std::rc::Rc;

use crate::entity::Entity;

#[derive(Clone)]
pub struct EntityCollection {
    entities: Vec<Rc<dyn Entity>>,
}

impl EntityCollection {
    pub fn new(entities: Vec<Rc<dyn Entity>>) -> Self {
        Self {
            entities: entities
        }
    }

    // pub fn entities(&self) -> impl Iterator<Item = Rc<dyn Entity>> + '_ {
    //     self.entities.iter().map(|entity| Rc::clone(entity))
    // }

    pub fn entities(&self) -> Vec<Rc<dyn Entity>> {
        self.entities.clone()
    }
}