use super::entity_collection::EntityCollection;

#[derive(Clone)]
pub struct WorldContext<'a> {
    pub entities: EntityCollection<'a>,
}

impl<'a> WorldContext<'a> {
    pub fn entities(&self) -> EntityCollection<'a> {
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