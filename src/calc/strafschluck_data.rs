use crate::calc::calculation::average;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::data::{Game, Team};

#[derive(Default)]
pub struct StrafschluckData {
    pub clean_hits: u32,
    pub clean_drinks: u32,
    pub straf_hits: u32,
    pub straf_drinks: u32,
    pub straf_schluecke: u32,
}

impl StrafschluckData {
    pub fn clean_average(&self) -> f32 {
        average(self.clean_hits, self.clean_drinks)
    }
    pub fn straf_average(&self) -> f32 {
        average(self.straf_hits, self.straf_drinks)
    }
    pub fn diff_average(&self) -> f32 {
        self.clean_average() - self.straf_average()
    }
    pub fn straf_per_finished(&self) -> f32 {
        average(self.straf_schluecke, self.straf_drinks)
    }
    pub fn effect_of_single_schluck(&self) -> f32 {
        self.diff_average() / self.straf_per_finished()
    }
    pub fn add_finished_beer(&mut self, clean: bool, hits_for_this_beer: u32, strafschluck_drank: u32, additional_hit_from_strafbeer: u32) {
        if clean {
            self.clean_drinks += 1;
            self.clean_hits += hits_for_this_beer + additional_hit_from_strafbeer;
        } else {
            self.straf_drinks += 1;
            self.straf_hits += hits_for_this_beer + additional_hit_from_strafbeer;
            self.straf_schluecke += strafschluck_drank;
        }
    }
    pub fn print(&self) {
        println!("Calculated Strafschluck Data:");
        println!("Clean: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", self.clean_drinks, self.clean_hits, self.clean_average());
        println!("Straf: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", self.straf_drinks, self.straf_hits, self.straf_average());
        println!("Effect of {} Strafschlucke: {:.3}\tNormalized for 1 Strafschluck per finished drink {:.3}", self.straf_schluecke, self.diff_average(), self.effect_of_single_schluck());
    }
}


#[derive(Default)]
pub struct StrafschluckCounter {
    pub hits: u32,
    pub schluck_drank: u32,
    pub clean: bool,
}

impl StrafschluckCounter {
    pub fn new() -> StrafschluckCounter {
        StrafschluckCounter {
            hits: 0,
            schluck_drank: 0,
            clean: true,
        }
    }
}
