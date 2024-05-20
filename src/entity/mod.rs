pub mod action;
pub mod alignment;
pub mod cooldown;
pub mod delay;
pub mod position;

use std::cell::BorrowMutError;
use std::cell::RefCell;
use std::rc::Rc;

pub use crate::entity::action::*;
pub use crate::entity::alignment::*;
pub use crate::entity::position::*;
pub use crate::entity::cooldown::*;
pub use crate::entity::delay::*;

pub use crate::entity_lib::EntityType;
use crate::world::world_context::WorldContext;


pub struct Entity {
    pub e_type: RefCell<EntityType>,
}

impl Entity {
    pub fn think(self: &Rc<Entity>, context: &WorldContext) -> Result<(Action, Delay, Cooldown), BorrowMutError> {
        match self.e_type.try_borrow_mut() {
            Result::Ok(mut e_type) => Ok(e_type.think(self, context)),
            Result::Err(e) => Err(e),
        }
    }

    pub fn position(self: &Rc<Entity>) -> Position {
        match self.e_type.try_borrow() {
            Result::Ok(mut e_type) => Ok(e_type.position(self)),
            Result::Err(e) => Err(e),
        }.expect("Entity already borrowed when getting position!")
    }

    pub fn alignment(self: &Rc<Entity>) -> Alignment {
        match self.e_type.try_borrow() {
            Result::Ok(mut e_type) => Ok(e_type.alignment(self)),
            Result::Err(e) => Err(e),
        }.expect("Entity already borrowed when getting alignment!")
    }
}