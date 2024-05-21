pub mod world_context;
pub mod entity_collection_ops;
pub mod entity_id;

mod command;
mod command_list;

use world_context::WorldContext;
use crate::entity::{Action, Cooldown, Delay, Entity};

use std::{iter, mem};
use std::rc::Rc;


pub struct World {
    entities: Vec<(Cooldown, Rc<Entity>)>,
    delayed_actions: Vec<(Delay, Action)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            delayed_actions: Vec::new(),
        }
    }

    pub fn from(entities: Vec<(Cooldown, Rc<Entity>)>) -> Self {
        Self {
            entities: entities,
            delayed_actions: Vec::new(),
        }
    }

    pub fn add_entity(self, entity: Rc<Entity>, cooldown: Cooldown) -> World {
        let entities = self.entities.iter()
            .chain(iter::once(&(cooldown, entity)))
            .map(|entity| entity.clone())
            .collect();
        
        World::from(entities)
    }

    pub fn step(&mut self) {
        let context = self.get_context();

        let new_actions = self.entities.iter_mut()
            .filter_map(|(cooldown, entity)| {
                match cooldown {
                    Cooldown::Finite(0) => {
                        let (action, delay, new_cooldown) = entity.think(&context);
                        *cooldown = new_cooldown;

                        Some((delay, action))
                    },
                    _ => None,
                }
            });

        let delayed_actions = mem::take(&mut self.delayed_actions);
        self.delayed_actions = delayed_actions.into_iter().chain(new_actions).collect();
    }

    pub fn get_context(&self) -> WorldContext {
        WorldContext {
            entities: Rc::new(
                self.entities.iter()
                    .map(|(_, entity)| entity.clone())
                    .collect()
            ),
        }
    }
}