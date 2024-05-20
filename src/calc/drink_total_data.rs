use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;
use crate::data::TeamMember;

pub struct PlayerDrinkingSpeed {
    pub drink_finished: DrinkFinishedStats,
    pub drink_avg: DrinkAvgStats,
    pub player_name: String
}

impl PlayerDrinkingSpeed {
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord= self.drink_avg.all_speed().partial_cmp(&other.drink_avg.all_speed());
        if ord.is_some() && ord.unwrap() == Equal {
            return self.drink_finished.pure_speed().partial_cmp(&other.drink_finished.pure_speed());
        }
        return ord;
    }
}