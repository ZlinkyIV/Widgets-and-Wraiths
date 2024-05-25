#[derive(Clone, Copy)]
pub struct Alignment {
    
}

impl Alignment {
    pub const NEUTRAL: Alignment = Alignment { };
}

impl Default for Alignment {
    fn default() -> Self {
        Self {  }
    }
}

pub fn are_allies(a: &Alignment, b: &Alignment) -> bool {
    true
}

pub fn are_enemies(a: &Alignment, b: &Alignment) -> bool {
    true
}