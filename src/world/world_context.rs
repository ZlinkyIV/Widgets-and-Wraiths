use super::entity_collection::EntityCollection;

pub struct WorldContext {
    pub entities: EntityCollection,
}

impl WorldContext {
    pub fn entities(&self) -> EntityCollection {
        self.entities.clone()
    }
}

// impl From<World> for WorldContext {
//     fn from(value: World) -> Self {
//         Self {
//             entities: value.entities
//         }
//     }
// }