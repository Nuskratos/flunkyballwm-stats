use crate::calc::accuracy_data::Accuracy;

pub struct EntityAccuracy{
    pub without_running: Accuracy,
    pub running: Accuracy,
}

impl EntityAccuracy{
    pub fn print(&self){
        let width = 30;
        println!("{:>width$} with running the previous round: {} ({:>3}/{:>3}) when not running: {} ({:>3}/{:>3})", self.running.named_entity.name, self.running.percentage_string(), self.running.hits, self.running.throws, self.without_running.percentage_string(), self.without_running.hits, self.without_running.throws);
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

}