pub mod cannon;

use std::rc::Rc;

pub use cannon::Cannon;

use crate::entity::{
    Entity,
    Action, Delay, Cooldown,
    Position, Alignment
};
use crate::world::world_context::WorldContext;


pub enum EntityType {
    Cannon(Cannon),
}

impl EntityType {
    pub fn think(&mut self, entity: &Rc<Entity>, context: &WorldContext) -> (Action, Delay, Cooldown) {
        match self {
            EntityType::Cannon(cannon) => Cannon::think(cannon, entity, context),
        }
    }

    pub fn position(&self, entity: &Rc<Entity>) -> Position {
        match self {
            EntityType::Cannon(cannon) => Cannon::position(cannon),
        }
    }

    pub fn alignment(&self, entity: &Rc<Entity>) -> Alignment {
        match self {
            EntityType::Cannon(cannon) => Cannon::alignment(cannon),
        }
    }
}