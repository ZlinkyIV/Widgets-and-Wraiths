use robots_and_skeletons::{
    entity::Position,
    entity_lib::cannon::Cannon,
    world::*
};

fn main() {
    let mut world = World::new();

    let mut cannon1 = Cannon::new(Position::Tangible(0, 0));
    let mut cannon2 = Cannon::new(Position::Tangible(0, 1));
    let mut cannon3 = Cannon::new(Position::Tangible(0, 2));
    
    world.add_entity(&mut cannon1);
    world.add_entity(&mut cannon2);
    world.add_entity(&mut cannon3);

    world.step();
    println!();
    world.step();
    println!();
    world.step();
}
