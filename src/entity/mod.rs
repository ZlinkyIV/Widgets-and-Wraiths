pub mod action;
pub mod alignment;
pub mod cooldown;
pub mod delay;
pub mod position;

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
    pub fn new(e_type: EntityType) -> Entity {
        Entity {
            e_type: RefCell::new(e_type),
        }
    }

    pub fn think(self: &mut Rc<Entity>, context: &WorldContext) -> (Action, Delay, Cooldown) {
        self.e_type.borrow_mut().think(self, context)
    }

    pub fn position(self: &Rc<Entity>) -> Position {
        self.e_type.borrow().position()
    }

    pub fn alignment(self: &Rc<Entity>) -> Alignment {
        self.e_type.borrow().alignment()
    }
}