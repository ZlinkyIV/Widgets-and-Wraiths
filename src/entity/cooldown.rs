#[derive(Clone, Copy)]
pub enum Cooldown {
    Finite(u16),
    Forever,
}