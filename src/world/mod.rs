pub mod world_context;
pub mod entity_collection;
pub mod entity_collection_ops;
pub mod entity_id;

mod command;
mod command_list;

use self::{entity_collection::EntityCollection, world_context::WorldContext};
use crate::entity::Entity;
use std::mem;
use std::rc::Rc;


pub struct World {
    entities: Vec<Rc<Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Rc<Entity>) {
        self.entities.push(entity);
    }

    pub fn step(&mut self) {
        let context = self.get_context();

        let actions = self.entities.iter()
            .map(|entity| {
                let (action, delay, cooldown) = entity.think(&context).expect("Already borrowed!");
                action
            });
        
        // let entities = mem::take(&mut self.entities);
        // self.entities = entities.into_iter()
        //     .map(|entity| {
        //         let action = entity.think(ctx.clone(), id);
        //         println!("Entity{} : {}", id, action);
        //         (id, entity)
        //     })
        //     .collect();
    }

    pub fn get_context(&self) -> WorldContext {
        WorldContext {
            entities: EntityCollection::new(Rc::new(self.entities))
        }
    }
}