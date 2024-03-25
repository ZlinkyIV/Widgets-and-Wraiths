use crate::entity::*;
use crate::world::entity_collection_ops::filter::*;

use self::alignment::Alignment;

pub struct Cannon {
    position: Position,
    alignment: Alignment,
}

impl Cannon {
    pub fn new() -> Self {
        Self {
            position: Position::Tangible(0, 0),
            alignment: Alignment {  },
        }
    }
}

impl Entity for Cannon {
    fn think(&mut self, ctx: WorldContext) -> Action {
        let closest_enemy_in_sight = ctx.entities()
            .filter(Inside(Circle { center: self.position, radius: 5 }))
            .filter(EnemiesOf(&self.alignment))
            .closest_to(&self.position);

        match closest_enemy_in_sight {
            None => Action::DoNothing(Cooldown::Finite(12)),
            Some(enemy) => Action::FireAt {
                target: enemy,
                // with: Projectile::new(),
            }
        }
    }

    fn position(&self) -> Position {
        self.position
    }
}