use {robots_and_skeletons::{entity_lib::cannon::Cannon, world::*}, std::rc::Rc};

fn main() {
    let mut world = World::new();

    let cannon = Cannon::new();
    
    world.add_entity(Rc::new(cannon));

    let ctx = world.get_context();
}
