pub mod action;
pub mod alignment;
pub mod cooldown;
pub mod position;

use dyn_clone::DynClone;

pub use crate::entity::action::*;
pub use crate::entity::alignment::*;
pub use crate::entity::position::*;
pub use crate::entity::cooldown::*;
pub use crate::world::entity_id::EntityID;

pub use crate::world::world_context::*;


pub trait Entity<'a>: DynClone {
    fn think(&mut self, cxt: WorldContext<'a>, my_id: EntityID) -> Action<'a> {
        Action::DoNothing(Cooldown::Forever)
    }

    fn position(&self) -> Position {
        Position::Intangible
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}

impl Clone for Box<dyn Entity<'_>> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

// impl Entity for Box<dyn Entity> {
//     fn think(&mut self, cxt: WorldContext) -> Action {
//         (*self).think(cxt)
//     }

//     fn position(&self) -> Position {
//         Position::Intangible
//     }

//     fn alignment(&self) -> Alignment {
//         Alignment::default()
//     }
// }