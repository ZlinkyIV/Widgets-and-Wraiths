use crate::{
    entity::{
        alignment::{self, Alignment}, EntityID, Position
    },
    world::entity_collection::EntityCollection
};

pub trait EntityCollectionFilter<'a> {
    fn apply(self, collection: EntityCollection<'a>) -> EntityCollection<'a>;
}

impl<'a> EntityCollection<'a> {
    pub fn filter<F>(self, filter: F) -> Self
        where F: EntityCollectionFilter<'a>
    {
        filter.apply(self)
    }
}


pub struct Not(pub EntityID);
impl<'a> EntityCollectionFilter<'a> for Not {
    fn apply(self, collection: EntityCollection<'a>) -> EntityCollection<'a> {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|(id, _)| {
                !(*id == self.0)
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

impl<'a> EntityCollectionFilter<'a> for Inside<Circle> {
    fn apply(self, collection: EntityCollection<'a>) -> EntityCollection<'a> {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|(_, entity)| {
                self.0.center.distance_to(&entity.position()) <= self.0.radius.into()
            })
            .collect();
        
        EntityCollection::new(qualifiers)
    }
}


pub struct EnemiesOf<'a>(pub &'a Alignment);

impl<'a> EntityCollectionFilter<'a> for EnemiesOf<'_> {
    fn apply(self, collection: EntityCollection<'a>) -> EntityCollection<'a> {
        let qualifiers = collection.entities()
            .into_iter()
            .filter(|(_, entity)| {
                alignment::are_enemies(&entity.alignment(), &self.0)
            })
            .collect();

        EntityCollection::new(qualifiers)
    }
}