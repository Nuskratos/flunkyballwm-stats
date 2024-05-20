use crate::calc::calculation::average;

#[derive(Default)]
pub struct StrafschluckData {
    pub clean_hits: u32,
    pub clean_drinks: u32,
    pub straf_hits: u32,
    pub straf_drinks: u32,
    pub straf_schluecke: u32,
}
impl StrafschluckData {
    pub fn clean_average(&self) -> f32{
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
    pub fn add_finished_beer(&mut self, clean : bool, hits_for_this_beer : u32, strafschluck_drank : u32, additional_hit_from_strafbeer : u32){
        if clean {
            self.clean_drinks +=1;
            self.clean_hits += hits_for_this_beer + additional_hit_from_strafbeer;
        }else {
            self.straf_drinks +=1;
            self.straf_hits += hits_for_this_beer + additional_hit_from_strafbeer;
            self.straf_schluecke += strafschluck_drank;
        }
    }
}


#[derive(Default)]
pub struct StrafschluckCounter{
    pub hits : u32,
    pub schluck_drank : u32,
    pub clean : bool
}
impl StrafschluckCounter{
    pub fn new() -> StrafschluckCounter{
        StrafschluckCounter{
            hits: 0,
            schluck_drank: 0,
            clean: true
        }
    }
}
