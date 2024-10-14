use crate::data::Team;

pub struct TeamRunningStatistics {
    pub speeds: Vec<(Team, RunningDiff)>,
    pub schluck_effect: f32
}
impl TeamRunningStatistics {
    pub fn print(&self){
        println!("Estimated Running Speeds:");
        println!("Speeds  are pretty experimental and most likely a poor substitution for an actual stopwatch.");
        println!("They are calculated, by counting the amount of times ran vs. the rounds where \n\
        enemies finished their drink in relation to their average vs the other teams. (similar to \n\
        the normal drinking speed calculation");
        println!("Strafschluck Effect used in this calculation: {:.3}", self.schluck_effect);
        let name_width = 27;
        let width = 16;
        println!("| {:^name_width$} | {:^width$} | {:^width$} | {:^width$} | ", "Name", "Round length", "Rounds ran", "Diff to expected");
        for (team, diff) in &self.speeds{
            println!("| {:>name_width$} | {:>width$.3} | {:>width$.3} | {:>width$.3} |", team.name, diff.round_length(), diff.run_amount, diff.diff_to_expected)
        }
    }
}

pub struct RunningDiff {
    pub baseline : f32,
    pub run_amount: f32,
    pub diff_to_expected: f32, // -1 if average would be 2 but 3 rounds were taken
}

impl RunningDiff {
    pub fn create() -> RunningDiff{
        RunningDiff{run_amount:0.0, diff_to_expected:0.0, baseline:1.0}
    }
    pub fn add(&mut self, other: &Self){
        self.run_amount += other.run_amount;
        self.diff_to_expected += other.diff_to_expected;
    }
    pub fn round_length(&self) -> f32{
        if self.run_amount == 0.0 {
            return 1.0;
        }
        self.baseline + (self.diff_to_expected / self.run_amount)
    }
}