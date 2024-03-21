use super::{
    command::Command,
    World,
};

pub struct CommandList {
    commands: Vec<Command>,
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn apply(self, world: &mut World) {
        
    }
}