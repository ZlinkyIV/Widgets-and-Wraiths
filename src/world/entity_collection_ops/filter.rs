use std::rc::Rc;

use crate::entity::{
    alignment::{are_enemies, Alignment},
    Entity,
    Position,
};
use crate::world::world_context::WorldContext;


pub trait Filter {
    fn apply(self, context: &WorldContext) -> WorldContext;
}

impl WorldContext {
    pub fn filter<F>(&self, filter: F) -> Self
        where F: Filter
    {
        filter.apply(self)
    }
}


// pub struct Is(pub Rc<Entity>);
// impl Filter for Is {
//     fn apply(self, context: WorldContext) -> WorldContext {
//         let entities = context.entities()
//             .filter(|entity| std::ptr::eq(&self.0, *entity))
//             .map(|entity| *entity)
//             .collect();
        
//         WorldContext {
//             entities: Rc::new(entities),
//         }
//     }
// }

pub struct Not(pub Rc<Entity>);
impl Filter for Not {
    fn apply(self, context: &WorldContext) -> WorldContext {
        let entities = context.entities()
            .filter(|entity| !std::ptr::eq(&self.0, entity))
            .collect();
        
        WorldContext {
            entities: Rc::new(entities),
        }
    }
}


pub struct Inside<S: Shape>(pub S);

pub trait Shape { }

pub struct Circle {
    pub center: Position,
    pub radius: u32,
}
impl Shape for Circle { }

impl Filter for Inside<Circle> {
    fn apply(self, collection: &WorldContext) -> WorldContext {
        let entities = collection.entities()
            .filter(|entity| {
                self.0.center.distance_to(&entity.position()) <= self.0.radius.into()
            })
            .collect();
        
        WorldContext {
            entities: Rc::new(entities),
        }
    }
}


pub struct EnemiesOf(pub Alignment);
impl Filter for EnemiesOf {
    fn apply(self, collection: &WorldContext) -> WorldContext {
        let entities = collection.entities()
            .filter(|entity| {
                are_enemies(&entity.alignment(), &self.0)
            })
            .collect();

        WorldContext {
            entities: Rc::new(entities),
        }
    }
}