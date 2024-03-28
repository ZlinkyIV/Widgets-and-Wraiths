use robots_and_skeletons::{
    entity::Position,
    entity_lib::cannon::Cannon,
    world::*
};

fn main() {
    let mut world = World::new();

    let mut cannon1 = Cannon::new(Position::Tangible(3, 2));
    let mut cannon2 = Cannon::new(Position::Tangible(2, 5));
    let mut cannon3 = Cannon::new(Position::Tangible(-4, 1));
    let mut cannon4 = Cannon::new(Position::Tangible(4, 0));
    let mut cannon5 = Cannon::new(Position::Tangible(-2, 0));
    
    world.add_entity(&mut cannon1);
    world.add_entity(&mut cannon2);
    world.add_entity(&mut cannon3);
    world.add_entity(&mut cannon4);
    world.add_entity(&mut cannon5);

    world.step();
    println!();
    world.step();
    println!();
    world.step();
    println!();
    world.step();
}
