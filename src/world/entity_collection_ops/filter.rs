use crate::{
    entity::{
        alignment,
        alignment::Alignment,
        Position
    },
    world::entity_collection::EntityCollection
};

pub trait EntityCollectionFilter {
    fn apply(self, collection: &EntityCollection) -> EntityCollection;
}

impl EntityCollection {
    pub fn filter<F>(&self, filter: F) -> Self
        where F: EntityCollectionFilter
    {
        filter.apply(self)
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
    fn apply(self, collection: &EntityCollection) -> EntityCollection {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|entity| {
                self.0.center.distance_to(&entity.position()) <= self.0.radius.into()
            })
            .collect();
        
        EntityCollection::new(qualifiers)
    }
}


pub struct EnemiesOf<'a>(pub &'a Alignment);

impl EntityCollectionFilter for EnemiesOf<'_> {
    fn apply(self, collection: &EntityCollection) -> EntityCollection {
        let qualifiers = collection.entities().into_iter()
            .filter(|entity| {
                alignment::are_enemies(&entity.alignment(), &self.0)
            })
            .collect();

        EntityCollection::new(qualifiers)
    }
}