use std::rc::Rc;

use crate::entity::*;

// #[derive(Clone, Copy)]
pub enum Action {
    DoNothing(Cooldown),
    FireAt {
        target: Rc<dyn Entity>,
        // with: Projectile,
    }
}