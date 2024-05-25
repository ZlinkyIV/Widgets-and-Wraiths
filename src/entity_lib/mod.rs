pub mod cannon;
pub mod cannon_ball;

use std::fmt::Display;
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
    #[inline(always)]
    pub fn think(&mut self, this: &Rc<Entity>, context: &WorldContext) -> (Action, Delay, Cooldown) {
        match self {
            EntityType::Cannon(cannon) => Cannon::think(cannon, this, context),
        }
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        match self {
            EntityType::Cannon(cannon) => Cannon::position(cannon),
        }
    }

    #[inline(always)]
    pub fn alignment(&self) -> Alignment {
        match self {
            EntityType::Cannon(cannon) => Cannon::alignment(cannon),
        }
    }
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cannon(_) => write!(f, "Cannon")
        }
    }
}