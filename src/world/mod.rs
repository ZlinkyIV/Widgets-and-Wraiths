pub mod world_context;
pub mod entity_collection;
pub mod entity_collection_ops;

mod command;
mod command_list;

use {self::{entity_collection::EntityCollection, world_context::WorldContext}, crate::{entity::Entity, entity_lib::cannon::Cannon}, std::{mem, rc::Rc}};

pub struct World {
    entities: Vec<Rc<dyn Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new()
        }
    }

    pub fn add_entity(&mut self, entity: Rc<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn step(&mut self) -> impl Iterator<Item = Rc<dyn Entity>> + '_ {
        let ctx = self.get_context();

        self.entities
            .iter_mut()
            .map(|entity|{
                mem::replace(entity, Rc::new(Cannon::new()))
            })
    }

    pub fn get_context(&self) -> WorldContext {
        WorldContext {
            entities: EntityCollection::new(self.entities.clone())
        }
    }
}