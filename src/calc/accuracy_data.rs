use std::fs::File;
use csv::Writer;
use crate::calc::accuracy_calc::calculate_throwing_accuracy;
use crate::data::NamedEntity;
use crate::team_player_data::NAME_WIDTH;
use crate::util::{open_writer, OpenedWriter};

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Copy, Clone)]
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
        if self.throws == 0 {
            return "?%".to_string();
        }
        format!("{:.2}%", self.percentage())
    }
    pub fn add_throw(&mut self, hit: bool) {
        self.throws += 1;
        if hit {
            self.hits += 1;
        }
    }
    pub fn new(named_entity: NamedEntity) -> Self{
        Self{named_entity, throws:0, hits:0}
    }
    pub fn print(&self){
        println!("{:<30} threw: {:>3} and hit: {:>3} which is {:<4.2}%", &self.named_entity.name, &self.throws, &self.hits, &self.percentage());
    }
    pub fn print_for_beer_impact(&self, i : usize){
        println!("Accuracy at: {:>2} beers drank: {:.2} ({:<3}/{:>3})", i, &self.percentage(), &self.hits, &self.throws)
    }
    fn seraialize_internal(&self, mut opened_writer:&mut OpenedWriter, write_alias:bool, file_prefix:&String){
        opened_writer.writer.write_record(&[file_prefix, &self.named_entity.name_or_alias(write_alias), &self.throws.to_string(), &self.hits.to_string(), &self.percentage_string()]);
    }
    pub fn merge(&mut self, other: &Self) {
        self.throws += other.throws;
        self.hits += other.hits;
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
        let filesufix= "throwing_accuracy.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Name", "Throws", "Hits", "Accuracy"]);
        }
        for entity in &self.accuracies{
            entity.seraialize_internal(&mut opened_writer, write_alias, &file_prefix);
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
        let filesufix= "enemy_accuracy.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Teamname", "Throws", "Hits", "Percentage"]);
        }
        for entity in &self.accuracies{
            entity.seraialize_internal(&mut opened_writer, write_alias, &file_prefix);        }
    }
}
pub struct FirstThrowAccuracy{
    pub accuracies: Vec<Accuracy>
}

impl FirstThrowAccuracy{
    pub fn print(&self){
        println!("First Throw Accuracy:");
        let width = 10;
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Teamname", "Throws", "Hits", "Percentage");
        for entity in &self.accuracies {
            entity.print();
        }
        println!();
    }

    pub fn serialize(&self, file_prefix:&String, date:&String) {
        let filesuffix = "further_first_throw_accuracy.csv".to_string();
        let real_writer = open_writer(date.to_string() + &filesuffix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string() + &date.to_string() + &filesuffix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    pub fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if ! opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Name", "Throws", "Hits", "Percentage"]);
        }
        for entity in &self.accuracies{
            entity.seraialize_internal(&mut opened_writer, write_alias, &file_prefix);
        }
    }
}