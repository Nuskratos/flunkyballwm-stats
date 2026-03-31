use crate::calc::calculation::{average};
use crate::util::{open_writer, OpenedWriter};

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
    pub fn straf_information(&self) ->String{
        format!("Effect of {} Strafschlucke: {:.3}", self.straf_schluecke, self.diff_average())
    }
    pub fn straf_normalized(&self)->String{
        format!("Effect of 1 Strafschluck to a finished drink {:.3}", self.effect_of_single_schluck())
    }
    pub fn print(&self) {
        println!("Calculated Strafschluck Data:");
        println!("Clean: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", self.clean_drinks, self.clean_hits, self.clean_average());
        println!("Straf: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", self.straf_drinks, self.straf_hits, self.straf_average());
        print!("{}", self.straf_information());
        print!("{}", self.straf_normalized());
        println!();
    }

    pub fn serialize(&self, file_prefix:&String, date: &String){
        let mut open_writer = open_writer(date.to_string()+"strafschluck.csv");
        if !open_writer.file_exists{
            open_writer.writer.write_record(&["HiddenPrefix",  "Type", "Drinks finished", "Hits required", "Average","Information"]);
        }
        open_writer.writer.write_record(&[file_prefix, "Clean", &self.clean_drinks.to_string(), &self.clean_hits.to_string(), &self.clean_average().to_string(), &self.straf_information()]);
        open_writer.writer.write_record(&[file_prefix, "With >=1 Strafschluck", &self.straf_drinks.to_string(), &self.straf_hits.to_string(), &self.straf_average().to_string(), &self.straf_normalized()]);
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
