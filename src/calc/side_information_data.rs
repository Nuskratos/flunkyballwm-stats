use std::fs::File;
use csv::Writer;
use crate::util::{open_writer, OpenedWriter};

pub struct SideSplit {
    pub left: SideInformation,
    pub right: SideInformation,
}

impl SideSplit {
    pub fn print(&self){
        println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
        println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", self.left.wins, self.left.points, self.left.hits as f32 / self.left.throws as f32 * 100.0, self.left.hits, self.left.throws, self.left.schluck, self.left.beer);
        println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", self.right.wins, self.right.points, self.right.hits as f32 / self.right.throws as f32 * 100.0, self.right.hits, self.right.throws, self.right.schluck, self.right.beer);
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let mut opened_writer = open_writer(date.to_string()+"side_information.csv");
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Side", "Wins", "Points", "Hits", "Throws", "Percentage", "Strafschluck", "Strafbier"]);
        }
        self.left.serialize(&mut opened_writer, file_prefix,"Left");
        self.right.serialize(&mut opened_writer, &file_prefix, "Right");
        }
}

#[derive(Default)]
pub struct SideInformation {
    pub wins: u32,
    pub points: u32,
    pub hits: u32,
    pub throws: u32,
    pub schluck: u32,
    pub beer: u32,
}

impl SideInformation {
    pub fn serialize(&self, opened_writer: &mut OpenedWriter, file_prefix: &String, side_string : &str){
        let percentage = self.hits as f32 / self.throws as f32 *100.0;
        let perc_str = format!("{:.2}%", percentage);
        opened_writer.writer.write_record(&[file_prefix, side_string, &self.wins.to_string(),&self.points.to_string(),
            &self.hits.to_string(), &self.throws.to_string(), &perc_str,&self.schluck.to_string(), &self.beer.to_string()]);
    }
}
