use {super::entity_id::EntityID, std::{collections::HashMap, rc::Rc}};

use crate::entity::Entity;

#[derive(Clone)]
pub struct EntityCollection<'a> {
    entities: HashMap<EntityID, Rc<dyn Entity<'a> + 'a>>,
}

impl<'a> EntityCollection<'a> {
    pub fn new(entities: HashMap<EntityID, Rc<dyn Entity<'a> + 'a>>) -> Self {
        // let entities = entities
        //     .iter()
        //     .map(|(id, entity)| (*id, (*entity).into()))
        //     .collect();
        
        Self {
            entities: entities,
        }
    }

    // pub fn entities(&self) -> impl Iterator<Item = Rc<dyn Entity>> + '_ {
    //     self.entities.iter().map(|entity| Rc::clone(entity))
    // }

    pub fn entities(&self) -> HashMap<EntityID, Rc<dyn Entity<'a> + 'a>> {
        self.entities.clone()
    }
}