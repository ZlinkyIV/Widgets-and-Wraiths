use std::rc::Rc;

use super::entity_collection::EntityCollection;

#[derive(Clone)]
pub struct WorldContext {
    pub entities: Rc<EntityCollection>,
}

impl WorldContext {
    pub fn entities(&self) -> Rc<EntityCollection> {
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