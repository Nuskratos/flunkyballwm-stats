use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

#[derive(Default)]
pub struct ChainInformation {
    pub current_hit: u32,
    pub current_miss: u32,
    pub total_hit: u32,
    pub total_miss: u32,
}

impl ChainInformation {
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = other.total_hit.partial_cmp(&self.total_hit);
        if ord.is_some() && ord.unwrap() == Equal {
            return self.total_miss.partial_cmp(&other.total_miss);
        }
        return ord;
    }
    pub fn create(hit: bool) -> ChainInformation {
        let mut ret: ChainInformation = Default::default();
        ret.throw(hit);
        ret
    }
    pub fn throw(&mut self, hit: bool) {
        if hit {
            self.hit();
        } else {
            self.miss();
        }
    }
    fn hit(&mut self) {
        self.current_hit += 1;
        if self.current_hit > self.total_hit {
            self.total_hit = self.current_hit;
        }
        self.current_miss = 0;
    }
    fn miss(&mut self) {
        self.current_miss += 1;
        if self.current_miss > self.total_miss {
            self.total_miss = self.current_miss;
        }
        self.current_hit = 0;
    }
}
