use crate::calc::accuracy_data::Accuracy;

pub struct EntityAccuracy{
    pub without_running: Accuracy,
    pub running: Accuracy,
}

pub struct AccuracyAfterRunningData{
    pub(crate) entities: Vec<EntityAccuracy>,
    pub average_running: Accuracy,
    pub average_without_running: Accuracy,
}

impl AccuracyAfterRunningData{
    pub fn print(&self){
        println!("Average without running: {:?}", self.average_without_running);
        println!("Average running: {:?}", self.average_running);
        for (i, entity) in self.entities.iter().enumerate(){
            println!("Entity {}: without running: {:?}, running: {:?}", i, entity.without_running, entity.running);
        }
    }

}