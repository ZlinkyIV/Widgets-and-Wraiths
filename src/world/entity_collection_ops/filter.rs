use std::rc::Rc;

use crate::{
    entity::{
        alignment::{Alignment, are_enemies}, Entity, Position
    },
    world::entity_collection::EntityCollection
};

pub trait EntityCollectionFilter {
    fn apply(self, collection: EntityCollection) -> EntityCollection;
}

impl EntityCollection {
    pub fn filter<F>(self, filter: F) -> Self
        where F: EntityCollectionFilter
    {
        filter.apply(self)
    }
}


pub struct Not(pub Rc<Entity>);
impl EntityCollectionFilter for Not {
    fn apply(self, collection: EntityCollection) -> EntityCollection {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|entity| {
                !(std::ptr::eq(&self.0, entity))
            })
            .collect();

        EntityCollection::new(qualifiers)
    }
}


pub struct Inside<S: Shape>(pub S);

pub trait Shape { }

pub struct Circle {
    pub center: Position,
    pub radius: u32,
}
impl Shape for Circle { }

impl EntityCollectionFilter for Inside<Circle> {
    fn apply(self, collection: EntityCollection) -> EntityCollection {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|entity| {
                self.0.center.distance_to(&entity.position()) <= self.0.radius.into()
            })
            .collect();
        
        EntityCollection::new(qualifiers)
    }
}


pub struct EnemiesOf(pub Alignment);

impl EntityCollectionFilter for EnemiesOf {
    fn apply(self, collection: EntityCollection) -> EntityCollection {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|entity| {
                are_enemies(&entity.alignment(), &self.0)
            })
            .collect();

        EntityCollection::new(qualifiers)
    }
}