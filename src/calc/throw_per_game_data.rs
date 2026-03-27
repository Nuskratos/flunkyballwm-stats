use std::cmp::Ordering;
use crate::data::{NamedEntity, TeamMember};

pub struct ThrowData {
    pub team: Vec<ThrowsPerGame>,
    pub player: Vec<ThrowsPerGame>,
    pub total_throws: u32
}
pub struct ThrowsPerGame {
    pub named_entity: NamedEntity,
    pub throws: u32,
    pub games: u32,
}

impl ThrowsPerGame {
    pub fn add_throws(&mut self, throws: u32) {
        self.throws += throws;
        self.games += 1;
    }
    pub fn new(entity:NamedEntity) -> ThrowsPerGame {
        ThrowsPerGame {
            throws: 0,
            games: 1,
            named_entity:entity
        }
    }
    pub fn average(&self) -> f32 {
        self.throws as f32 / self.games as f32
    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        self.average().partial_cmp(&other.average())
    }

}