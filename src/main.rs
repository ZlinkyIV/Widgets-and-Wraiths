use std::rc::Rc;

use robots_and_skeletons::world::*;
use robots_and_skeletons::entity::{
    Entity,
    Cooldown, Position,
};
use robots_and_skeletons::entity_lib::Cannon;

fn main() {
    let cannon1 = Rc::new(Entity::new(
        Cannon::new(Position::Tangible(3, 2)).into()
    ));
    let cannon2 = Rc::new(Entity::new(
        Cannon::new(Position::Tangible(2, 5)).into()
    ));
    let cannon3 = Rc::new(Entity::new(
        Cannon::new(Position::Tangible(-4, 1)).into()
    ));
    let cannon4 = Rc::new(Entity::new(
        Cannon::new(Position::Tangible(4, 0)).into()
    ));
    let cannon5 = Rc::new(Entity::new(
        Cannon::new(Position::Tangible(-2, 0)).into()
    ));
    
    let mut world = World::new()
        .add_entity(cannon1, Cooldown::Time(0))
        .add_entity(cannon2, Cooldown::Time(0))
        .add_entity(cannon3, Cooldown::Time(0))
        .add_entity(cannon4, Cooldown::Time(0))
        .add_entity(cannon5, Cooldown::Time(0));

    world.step();
    println!();
    world.step();
    println!();
    world.step();
    println!();
    world.step();
}
