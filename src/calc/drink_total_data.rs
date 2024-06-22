use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;

pub struct PlayerDrinkingSpeed {
    pub drink_finished: DrinkFinishedStats,
    pub drink_avg: DrinkAvgStats,
    pub player_name: String,
}

impl PlayerDrinkingSpeed {
    pub fn set_to_5_speed(&mut self){
        self.drink_finished.all_drinks = 1;
        self.drink_finished.all_hits = 5.0;
        self.drink_finished.pure_drinks = 1;
        self.drink_finished.pure_hits = 5;
        self.drink_avg.all_drinks = 1;
        self.drink_avg.all_hits = 5.0;
        self.drink_avg.pure_drinks = 1;
        self.drink_avg.pure_hits = 5;

    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = self.drink_avg.all_speed().partial_cmp(&other.drink_avg.all_speed());
        if ord.is_some() && ord.unwrap() == Equal {
            return self.drink_finished.pure_speed().partial_cmp(&other.drink_finished.pure_speed());
        }
        return ord;
    }
    pub fn print(&self, name_width: usize, width:usize) {
        let pure_finished = format!("{:>.2} ({:>4.2} / {:>2})", self.drink_finished.pure_speed(), self.drink_finished.pure_hits, self.drink_finished.pure_drinks);
        let pure_average = format!("{:>.2} ({:>4.2} / {:>2})", self.drink_avg.pure_speed(), self.drink_avg.pure_hits, self.drink_avg.pure_drinks);
        let all_finished = format!("{:>.2} ({:>4.2} / {:>2})", self.drink_finished.all_speed(), self.drink_finished.all_hits, self.drink_finished.all_drinks);
        let all_average = format!("{:>.2} ({:>4.2} / {:>2})", self.drink_avg.all_speed(), self.drink_avg.all_hits, self.drink_avg.all_drinks);
        println!("| {:>name_width$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |", self.player_name, pure_finished, pure_average, all_finished, all_average);
    }
}

pub struct DrinkingSpeedVec {
    pub speeds: Vec<PlayerDrinkingSpeed>,
    pub schluck_effect: f32
}
impl DrinkingSpeedVec{
    pub fn print(&self) {
        println!("Different drinking speed metrics:
Pure finished: Finished drinks without StrafSchluck
Pure average: Finished drinks, not-finished rounds with >=flat(Pure) count as (rounds+1) - no StrafSchluck
All finished: Finished drinks with Strafschluck
All average: Finished drinks, not-finished with (rounds >=flat(All finished)) count as (rounds+1) including Strafschlucks
for all above: StrafBeer counts as finished in +1 rounds");
        println!("Selected Strafschluck effect: {:.3} rounds", self.schluck_effect);
        let n_c = 10;
        let width = 17;
        println!("{:-<94}", "-");
        println!("| {:^n_c$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Player", "Pure finished", "Pure average", "All finished", "All average");
        println!("{:-<94}", "-");
        for player in &self.speeds {
            player.print(n_c, width);
        }
    }
}