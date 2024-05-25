use std::rc::Rc;

use crate::entity::*;
use crate::world::world_context::WorldContext;

#[derive(Clone)]
pub struct CannonBall {
    position: Position,
    direction: Direction,
    distance: Distance,
    speed: Speed,
    damage_amount: u32,
    damage_radius: f32,
}

impl CannonBall {
    pub fn new(position: Position, direction: Direction, distance: Distance, speed: Speed, damage_amount: u32, damage_radius: f32) -> CannonBall {
        CannonBall {
            position,
            direction,
            distance,
            speed,
            damage_amount,
            damage_radius,
        }
    }
}

impl CannonBall {
    pub fn think(&mut self, this: &Rc<Entity>, _: &WorldContext) -> (Action, Delay, Cooldown) {
        // let new_position = match (self.position, self.destination) {
        //     (Position::Intangible, _) | (_, Position::Intangible) =>
        //         return (Action::Destroy(this), Delay::Time(0), Cooldown::Forever),
        //     (Position::Tangible(cur_x, cur_y), Position::Tangible(des_x, des_y)) => {
        //         let (dir_x, dir_y) = normalize((des_x - cur_x, des_y - cur_y));
        //         let (new_x, new_y) = (
        //             dir_x * (self.speed as f32) + (cur_x as f32),
        //             dir_y * (self.speed as f32) + (cur_y as f32),
        //         );
        //         (new_x as i16, new_y as i16)
        //     },
        // };

        // (Action::MoveTo(this, new_position, 10))
    }

    pub fn spawn(&mut self, this: &Rc<Entity>, _: &WorldContext) -> (Action, Delay, Cooldown) {
        // let ticks_to_dest = self.direction / self.speed;
        // let damage_square_bottom_right = self.position + (self.direction.normal() * self.damage_radius / 2);
        // let damage_square_top_left = self.position + (-self.direction.normal() * self.damage_radius / 2);

        // [
        //     (
        //         Effect::MoveInDirection(self.direction, self.speed),
        //         Effect::AreaDamage(Shape::Square(damage_square_top_left, damage_square_bottom_right), self.damage_amount),
        //     ).repeat_times(ticks_to_dest),
        //     Effect::DestroyEntity(this),
        // ].into()
        
        // let distance_to_destination = match (self.position, self.destination) {
        //     (Position::Intangible, _) | (_, Position::Intangible) =>
        //         return (Action::Destroy(this), Delay::Time(0), Cooldown::Forever),
        //     (Position::Tangible(cur_x, cur_y), Position::Tangible(des_x, des_y)) =>
        //         ((cur_x - des_x).abs(), (cur_y - des_y).abs()),
        // };
        let distance_to_destination = self.position.distance_to(&self.destination).dist();

        let distance_to_destination = match distance_to_destination {
            None => return (Action::Destroy(this), Delay::Time(0), Cooldown::Forever),
            Some(dist) => dist
        };

        let ticks_to_destination = (distance_to_destination) / (self.speed as f64);

        (
            Action::Sequence([
                Action::MoveTowards(this.clone(), self.destination, ticks_to_destination),
                Action::AddEffect(Effect::Damage(self.destination, self.damage_radius, self.damage_amount)),
                Action::Destroy(this.clone()),
            ]),
            Delay::Time(0),
            Cooldown::Forever,
        )
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        self.position
    }

    #[inline(always)]
    pub fn alignment(&self) -> Alignment {
        Alignment::NEUTRAL
    }
}