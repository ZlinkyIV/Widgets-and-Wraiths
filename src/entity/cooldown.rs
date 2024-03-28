#[derive(Clone, Copy, Debug)]
pub enum Cooldown {
    Finite(u16),
    Forever,
}