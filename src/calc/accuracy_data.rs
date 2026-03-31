use std::fs::File;
use csv::Writer;
use crate::calc::calculation::open_writer;
use crate::data::NamedEntity;
use crate::team_player_data::NAME_WIDTH;

pub struct Accuracy {
    pub named_entity: NamedEntity,
    pub throws: u32,
    pub hits: u32,
}

impl Accuracy {
    pub fn percentage(&self) -> f32 {
        self.hits as f32 / self.throws as f32 * 100.0
    }
    pub fn percentage_string(&self)->String{
        format!("{:.2}%", self.percentage())
    }
    pub fn add_throw(&mut self, hit: bool) {
        self.throws += 1;
        if hit {
            self.hits += 1;
        }
    }
    pub fn default(named_entity: NamedEntity)-> Self{
        Self{named_entity:named_entity, throws:0, hits:0}
    }
    pub fn print(&self){
        println!("{:<30} threw: {:>3} and hit: {:>3} which is {:<4.2}%", &self.named_entity.name, &self.throws, &self.hits, &self.percentage());
    }
}

pub struct EntityAccuracy{
    pub accuracies: Vec<Accuracy>
}
impl EntityAccuracy{
    pub fn print(&self){
        println!("Throwing Accuracy:");
        let width = 16;
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Teamname", "Throws", "Hits", "Percentage");
        for team in &self.accuracies {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team.named_entity.name, team.throws, team.hits, team.percentage());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let (mut writer, file_exists) = open_writer(date.to_string()+"throwing_accuracy.csv");
        if !file_exists{
            writer.write_record(&["HiddenPrefix", "Name", "Throws", "Hits", "Accuracy"]);
        }
        for entity in &self.accuracies{
            writer.write_record(&[file_prefix, entity.named_entity.name, &entity.throws.to_string(), &entity.hits.to_string(), &entity.percentage_string()]);
        }
    }
}

pub struct EnemyAccuracy{
    pub accuracies: Vec<Accuracy>
}

impl EnemyAccuracy{
    pub fn print(&self){
        println!("Enemy Accuracy:");
        let width = 10;
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Teamname", "Throws", "Hits", "Percentage");
        for team in &self.accuracies {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team.named_entity.name, team.throws, team.hits, team.percentage());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let (mut writer, file_exists) = open_writer(date.to_string()+"enemy_accuracy.csv");
        if !file_exists{
            writer.write_record(&["HiddenPrefix", "Teamname", "Thorws", "Hits", "Percentage"]);
        }
        for team in &self.accuracies{
            writer.write_record(&[file_prefix, team.named_entity.name, &team.throws.to_string(), &team.hits.to_string(), &team.percentage_string()]);
        }
    }
}