use std::fmt::Display;
use std::rc::Rc;

use crate::entity::*;

#[derive(Clone)]
pub enum Action {
    DoNothing,
    MoveTowards(Rc<Entity>, Position, u16),
    Spawn(Rc<Entity>),
    Destroy(Rc<Entity>),
    AddEffect(Effect),
    Sequence(Box<[Action]>),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoNothing => write!(f, "Do Nothing for"),
            Self::Spawn(entity) => write!(f, "Spawn entity {}", entity.e_type.borrow()),
        }
    }
}