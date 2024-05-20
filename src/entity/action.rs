use std::fmt::Display;
use std::{fmt::Debug, rc::Rc};

use crate::entity::*;

// #[derive(Clone, Copy)]
pub enum Action {
    DoNothing,
    FireAtEntity(Rc<Entity>)
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoNothing => write!(f, "Do Nothing for"),
            Self::FireAtEntity(target) => write!(f, "Fire At Entity at {:?}", target.position()),
        }
    }
}

// impl Debug for Action<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::DoNothing(cooldown) => f.debug_tuple("DoNothing").field(cooldown).finish(),
//             Self::FireAtEntity { target } => f.debug_tuple("FireAt").field(&target.position()).finish(),
//         }
//     }
// }