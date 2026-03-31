use std::fmt::format;
use crate::data::NamedEntity;
use crate::util::{open_writer, print_line_break, OpenedWriter};

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

pub struct FirstTeamThrowsStatistics {
    pub teams: Vec<NamedThrows>
}

impl FirstTeamThrowsStatistics {
    pub fn print(&self){
        println!("Teamname:                   | Going first | Won as first | Win% First | Going Second | Won as Second | Win% Second");
        for elem in &self.teams {
            println!("{:<27} | {:<11} | {:<12}  | {:<12}  | {:<12} | {:<12} | {:<8}", elem.named_entity.name, elem.throw_data.threw_first, elem.throw_data.won_first, elem.throw_data.win_perc_string(),elem.throw_data.threw_second, elem.throw_data.won_second, elem.throw_data.second_perc_string());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let filesufix= "first_throw.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Teamname", "Going First", "Won as First", "Win% First","Going Second", "Won as Second", "Win% second"]);
        }
        for team in &self.teams{
            opened_writer.writer.write_record(&[file_prefix, &team.named_entity.name_or_alias(write_alias), &team.throw_data.threw_first.to_string(), &team.throw_data.won_first.to_string(), &team.throw_data.win_perc_string(), &team.throw_data.threw_second.to_string(), &team.throw_data.won_second.to_string(), &team.throw_data.second_perc_string()]);
        }
    }
}

pub struct FirstThrowStatistic{
    pub first_throw_win :u32,
    pub games: u32,
    pub first_hit:u32,
    pub first_hit_win_amount:u32,
    pub first_miss:u32,
    pub first_miss_win_amount:u32
}

impl FirstThrowStatistic{
    pub fn first_throw_win_string(&self)->String{
        format!("{:.2}", self.first_throw_win as f32 / self.games as f32)
    }
    pub fn first_hit_win_string(&self)->String{
        format!("{:.2}", self.first_hit_win_amount as f32 / self.first_hit as f32)
    }
    pub fn first_miss_win_string(&self)->String{
        format!("{:.2}", self.first_miss_win_amount as f32 / self.first_miss as f32)
    }
    pub fn general_first_string(&self)->String{
        format!("In {} Spielen hat das Team mit dem 1. Wurfrecht {} mal gewonnen. Das sind {}%", self.games, self.first_throw_win, self.first_throw_win_string())
    }
    pub fn hit_first_string(&self)->String{
        format!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst getroffen. Dabei {} mal gewonnen. Das sind {}%", self.first_hit, self.first_hit_win_amount, self.first_hit_win_string())
    }
    pub fn miss_first_string(&self)->String{
        format!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst verfehlt. Dabei {} mal gewonnen. Das sind {}%", self.first_miss,self.first_miss_win_amount, self.first_miss_win_string())
    }
    pub fn print(&self){
        println!("{}", self.general_first_string());
        println!("{}", self.hit_first_string());
        println!("{}", self.miss_first_string());
        println!();
    }

    pub fn serialize(&self, file_prefix:&String, date: &String){
        let mut opened_writer = open_writer(date.to_string()+"general_first_throw.csv");
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix",  "Erstwurfeffekt"]);
        }
        opened_writer.writer.write_record(&[file_prefix, &self.general_first_string()]);
        opened_writer.writer.write_record(&[file_prefix, &self.hit_first_string()]);
        opened_writer.writer.write_record(&[file_prefix, &self.miss_first_string()]);
    }
}