#[derive(Clone, Copy, Debug)]
pub enum Cooldown {
    Time(u16),
    Forever,
}