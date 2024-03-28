pub mod world_context;
pub mod entity_collection;
pub mod entity_collection_ops;
pub mod entity_id;

mod command;
mod command_list;

use self::{entity_collection::EntityCollection, entity_id::EntityID, world_context::WorldContext};
use crate::entity::Entity;
use std::{collections::HashMap, mem, rc::Rc};

// pub fn de_mutate<T: ?Sized>(thing: &mut T) -> &T {
//     &thing
// }

pub struct World<'a> {
    entities: HashMap<EntityID, &'a mut dyn Entity<'a>>,
    current_index: u32, // This is a bit hacky, but it'll work for now
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            current_index: 0,
        }
    }

    pub fn add_entity(&mut self, entity: &'a mut dyn Entity<'a>) {
        self.current_index += 1;
        let id = EntityID::from_raw_and_generation(self.current_index, 0);

        let old_value_if_any = self.entities.insert(id, entity);

        if let Some(_) = old_value_if_any {
            panic!("Duplicated id!")
        }
    }

    pub fn step(&mut self) {
        let ctx = self.get_context();
        
        let entities = mem::take(&mut self.entities);
        self.entities = entities.into_iter()
            .map(|(id, entity)| {
                let action = entity.think(ctx.clone(), id);
                println!("Entity{} : {}", id, action);
                (id, entity)
            })
            .collect();
    }

    pub fn get_context(&self) -> WorldContext<'a> {
        let entities = self.entities.iter()
            .map(|(id, entity_ref)| {
                (*id, Rc::from(dyn_clone::clone_box(*entity_ref)))
            })
            .collect();

        WorldContext {
            entities: EntityCollection::new(entities)
        }
    }
}