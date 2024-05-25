pub enum Effect {
    MoveInDirection(Direction, f32),
    Damage(Position, f32, u32),
}