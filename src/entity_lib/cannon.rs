use std::rc::Rc;

use crate::entity::*;
use crate::world::entity_collection_ops::filter::*;
use crate::world::world_context::WorldContext;

use self::alignment::Alignment;

#[derive(Clone)]
pub struct Cannon {
    position: Position,
    alignment: Alignment,
}

impl Cannon {
    pub fn new(position: Position) -> Self {
        Self {
            position: position,
            alignment: Alignment {  },
        }
    }
}

impl Cannon {
    pub fn think(&mut self, this: &Rc<Entity>, context: &WorldContext) -> (Action, Delay, Cooldown) {
        let closest_enemy_in_sight = context
            .filter(Inside(Circle { center: self.position, radius: 5 }))
            .filter(EnemiesOf(self.alignment))
            .filter(Not(this.clone()))
            .closest_to(&self.position);

        // self.position = match self.position {
        //     Position::Tangible(x, y) => Position::Tangible(x - 1, y * 2 - 1),
        //     Position::Intangible => Position::Intangible,
        // };

        match closest_enemy_in_sight {
            None => (Action::DoNothing, Delay::Time(0), Cooldown::Finite(12)),
            Some(enemy) => (Action::FireAtEntity(enemy), Delay::Time(0), Cooldown::Finite(12)),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}