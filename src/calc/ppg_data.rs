use std::cmp::Ordering;

pub struct PpgHolder {
    pub points: u32,
    pub games: u32,
}

impl PpgHolder {
    pub fn ppg(&self) -> f32 {
        self.points as f32 / self.games as f32
    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = other.ppg().partial_cmp(&self.ppg());
        return ord;
    }
    pub fn new() -> PpgHolder {
        PpgHolder { points: 0, games: 1 }
    }
}