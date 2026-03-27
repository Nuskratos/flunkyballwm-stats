use crate::calc::calculation::{open_writer, percentage};
use crate::data::NamedEntity;
use crate::util::print_line_break;

pub struct FirstThrows {
    pub threw_first: u32,
    pub won_first: u32,
    pub threw_second: u32,
    pub won_second: u32,
}
impl FirstThrows {
    pub fn default() -> Self {
        Self {
            threw_first: 0,
            won_first: 0,
            threw_second: 0,
            won_second: 0,
        }
    }
    pub fn win_perc(&self)->f32{
        self.won_first as f32 / self.threw_first as f32
    }
    pub fn win_perc_string(&self)->String{
        format!("{:.2}", self.win_perc())
    }
    pub fn second_perc(&self)->f32{
        self.won_second as f32 / self.threw_second as f32
    }
    pub fn second_perc_string(&self)->String{
        format!("{:.2}", self.second_perc())
    }
}
pub struct NamedThrows{
    pub named_entity: NamedEntity,
    pub throw_data: FirstThrows
}

pub struct FirstThrowsStatistics{
    pub teams: Vec<NamedThrows>
}

impl FirstThrowsStatistics{
    pub fn print(&self){
        println!("Teamname:                   | Going first | Won as first | Win% First | Going Second | Won as Second | Win% Second");
        for elem in &self.teams {
            println!("{:<27} | {:<11} | {:<12}  | {:<12}  | {:<12} | {:<12} | {:<8}", elem.named_entity.name, elem.throw_data.threw_first, elem.throw_data.won_first, elem.throw_data.win_perc_string(),elem.throw_data.threw_second, elem.throw_data.won_second, elem.throw_data.second_perc_string());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let (mut writer, file_exists) = open_writer(date.to_string()+"first_throw.csv");
        if !file_exists{
            writer.write_record(&["HiddenPrefix", "Teamname", "Going First", "Won as First", "Win% First","Going Second", "Won as Second", "Win% second"]);
        }
        for team in &self.teams{
            writer.write_record(&[file_prefix, team.named_entity.name, &team.throw_data.threw_first.to_string(), &team.throw_data.won_first.to_string(), &team.throw_data.win_perc_string(), &team.throw_data.threw_second.to_string(), &team.throw_data.won_second.to_string(), &team.throw_data.second_perc_string()]);
        }
    }
}