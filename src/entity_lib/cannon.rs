use crate::entity::*;
use crate::world::entity_collection_ops::filter::*;

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

impl<'a> Entity<'a> for Cannon {
    fn think(&mut self, ctx: WorldContext<'a>, my_id: EntityID) -> Action<'a> {
        let closest_enemy_in_sight = ctx.entities()
            .filter(Inside(Circle { center: self.position, radius: 5 }))
            .filter(EnemiesOf(&self.alignment))
            .filter(Not(my_id))
            .closest_to(&self.position);

        self.position = match self.position {
            Position::Tangible(x, y) => Position::Tangible(x, y * 10),
            Position::Intangible => Position::Intangible,
        };

        // let _ = std::mem::replace(self, Self {
        //     position: Position::Intangible,
        //     alignment: self.alignment,
        // });

        match closest_enemy_in_sight {
            None => Action::DoNothing(Cooldown::Finite(12)),
            Some(enemy) => Action::FireAtEntity {
                target: enemy,
                // with: Projectile::new(),
            }
        }
    }

    fn position(&self) -> Position {
        self.position
    }
}