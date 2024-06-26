pub struct DrinkAvgStats {
    pub pure_drinks : u32,
    pub pure_hits : u32,
    pub all_drinks : u32,
    pub all_hits : f32
}
impl DrinkAvgStats {
    pub fn new() -> DrinkAvgStats {
        DrinkAvgStats {
            pure_drinks:0,
            pure_hits:0,
            all_drinks:0,
            all_hits:0.0
        }
    }
    pub fn p_avg(&mut self, beers: u32, rounds: u32) {
        self.pure_drinks += beers;
        self.pure_hits += rounds;
    }
    pub fn a_avg(&mut self, beers: u32, rounds: f32) {
        self.all_drinks += beers;
        self.all_hits += rounds;
    }
    pub fn pure_speed(&self) -> f32 {
        self.pure_hits as f32/ self.pure_drinks as f32
    }
    pub fn all_speed(&self) -> f32 {
        self.all_hits / self.all_drinks as f32
    }
}
