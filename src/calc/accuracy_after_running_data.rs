use std::fs::File;
use crate::calc::accuracy_data::Accuracy;
use crate::util::{open_writer, OpenedWriter};

pub struct EntityAccuracy{
    pub without_running: Accuracy,
    pub running: Accuracy,
}

impl EntityAccuracy{
    pub fn after_running_string(&self)->String{
        format!("{} ({:>3}/{:>3})", self.running.percentage_string(), self.running.hits, self.running.throws)
    }
    pub fn not_after_running_string(&self)->String{
        format!("{} ({:>3}/{:>3})", self.without_running.percentage_string(), self.without_running.hits, self.without_running.throws)
    }
    pub fn difference_string(&self)->String{
        format!("{:.2}%", self.difference())
    }
    pub fn print(&self){
        let width = 30;
        println!("{:>width$} with running the previous round: {} when not running: {} Difference: {}", self.running.named_entity.name, self.after_running_string(), self.not_after_running_string(), self.difference_string());
    }
    pub fn difference(&self)->f32{
        self.running.percentage() - self.without_running.percentage()
    }
}

pub struct AccuracyAfterRunningData{
    pub(crate) entities: Vec<EntityAccuracy>,
    pub(crate) average: EntityAccuracy,
}

impl AccuracyAfterRunningData{
    pub fn print(&self){
        self.average.print();
        for entity in self.entities.iter(){
            entity.print();
        }
    }
    pub fn serialize(&self, file_prefix: &String, date: &String){
        let filesuffix = "running_impact_on_accuracy.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesuffix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesuffix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }
    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix: &str){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Name", "After Running", "Without Running", "Difference"]);
        }
        opened_writer.writer.write_record(&[file_prefix, &self.average.running.named_entity.name_or_alias(write_alias), &self.average.after_running_string(), &self.average.not_after_running_string(), &self.average.difference_string()]);
        for entry in &self.entities{
            opened_writer.writer.write_record(&[file_prefix, &entry.running.named_entity.name_or_alias(write_alias), &entry.after_running_string(), &entry.not_after_running_string(), &entry.difference_string()]);
        }
    }
}