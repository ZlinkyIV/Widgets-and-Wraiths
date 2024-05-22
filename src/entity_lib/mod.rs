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

impl From<Cannon> for EntityType {
    fn from(value: Cannon) -> Self {
        Self::Cannon(value)
    }
}

impl EntityType {
    pub fn think(&mut self, this: &Rc<Entity>, context: &WorldContext) -> (Action, Delay, Cooldown) {
        match self {
            EntityType::Cannon(cannon) => Cannon::think(cannon, this, context),
        }
    }

    pub fn position(&self) -> Position {
        match self {
            EntityType::Cannon(cannon) => Cannon::position(cannon),
        }
    }

    pub fn alignment(&self) -> Alignment {
        match self {
            EntityType::Cannon(cannon) => Cannon::alignment(cannon),
        }
    }
}