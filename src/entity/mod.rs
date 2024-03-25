pub mod action;
pub mod alignment;
pub mod cooldown;
pub mod position;

pub use crate::entity::action::*;
pub use crate::entity::alignment::*;
pub use crate::entity::position::*;
pub use crate::entity::cooldown::*;

pub use crate::world::world_context::*;


pub trait Entity {
    fn think(&mut self, cxt: WorldContext) -> Action {
        Action::DoNothing(Cooldown::Forever)
    }

    fn position(&self) -> Position {
        Position::Intangible
    }

    fn alignment(&self) -> Alignment {
        Alignment::default()
    }
}