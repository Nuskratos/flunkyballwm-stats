use Ordering::{Greater, Less};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fs::File;
use csv::Writer;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;
use crate::data::NamedEntity;
use crate::util::{open_writer, OpenedWriter};

pub struct PlayerDrinkingSpeed {
    pub drink_finished: DrinkFinishedStats,
    pub drink_avg: DrinkAvgStats,
    pub player_entity: NamedEntity,
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
    match (
        self.drink_finished.all_drinks == 0,
        other.drink_finished.all_drinks == 0,
    ) {
        (true, true) => return Some(Ordering::Equal),
        (true, false) => return Some(Ordering::Greater),
        (false, true) => return Some(Ordering::Less),
        (false, false) => {}
    }

    match self
        .drink_avg
        .all_speed()
        .partial_cmp(&other.drink_avg.all_speed())
    {
        Some(Ordering::Equal) => self
            .drink_finished
            .pure_speed()
            .partial_cmp(&other.drink_finished.pure_speed()),
        Some(ordering) => Some(ordering),
        None => Some(Ordering::Equal),
    }
}
    pub fn pure_finished(&self)->String{
        format!("{:>.2} ({:>4.2} / {:>2})", self.drink_finished.pure_speed(), self.drink_finished.pure_hits, self.drink_finished.pure_drinks)
    }
    pub fn pure_average(&self) ->String{
        format!("{:>.2} ({:>4.2} / {:>2})", self.drink_avg.pure_speed(), self.drink_avg.pure_hits, self.drink_avg.pure_drinks)
    }
    pub fn all_finished(&self)->String{
        format!("{:>.2} ({:>4.2} / {:>2})", self.drink_finished.all_speed(), self.drink_finished.all_hits, self.drink_finished.all_drinks)
    }
    pub fn all_average(&self)->String{
        format!("{:>.2} ({:>4.2} / {:>2})", self.drink_avg.all_speed(), self.drink_avg.all_hits, self.drink_avg.all_drinks)
    }
    pub fn print(&self, name_width: usize, width:usize) {
        let pure_finished = self.pure_finished();
        let pure_average = self.pure_finished();
        let all_finished = self.all_finished();
        let all_average = self.all_average();
        println!("| {:>name_width$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |", self.player_entity.name, pure_finished, pure_average, all_finished, all_average);
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
        println!();
    }

    pub fn serialize(&self, file_prefix:&String, date: &String){
        let filesufix= "drinking_speed.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Player", "Pure finished", "Pure average", "All finished", "All average"]);
        }
        for playerdrink_speed in &self.speeds{
            opened_writer. writer.write_record(&[file_prefix, &playerdrink_speed.player_entity.name_or_alias(write_alias), &playerdrink_speed.pure_finished(), &playerdrink_speed.pure_average(), &playerdrink_speed.all_finished(), &playerdrink_speed.all_average()]);
        }
    }
}