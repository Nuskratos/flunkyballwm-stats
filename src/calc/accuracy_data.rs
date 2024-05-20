pub struct Accuracy {
    pub name: &'static str,
    pub throws: u32,
    pub hits: u32,
}

impl Accuracy {
    pub fn percentage(&self) -> f32 {
        self.hits as f32 / self.throws as f32 * 100.0
    }
    pub fn add_throw(&mut self, hit: bool) {
        self.throws += 1;
        if hit {
            self.hits += 1;
        }
    }
}

pub fn print_accuracy(accuracy: &Accuracy) {
    println!("{:<30} threw: {:>3} and hit: {:>3} which is {:<4.2}%", accuracy.name, accuracy.throws, accuracy.hits, accuracy.percentage());
}