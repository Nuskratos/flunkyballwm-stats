use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Penalties {
    pub beers: u32,
    pub schlucke: u32,
    pub games : u32
}
impl Penalties {

    pub fn spg(&self) -> f32 {
        self.schlucke as f32 / self.games as f32
    }
    pub fn bpg(&self) -> f32 {
        self.beers as f32 / self.games as f32
    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord= self.spg().partial_cmp(&other.spg());
        if ord.is_some() && ord.unwrap() == Equal {
            return self.bpg().partial_cmp(&other.bpg());
        }
        return ord;
    }
    pub fn create_beer() -> Penalties {
        let mut ret : Penalties = Default::default();
        ret.beers = 1;
        ret
    }
    pub fn create_schluck() -> Penalties {
        let mut ret : Penalties = Default::default();
        ret.schlucke = 1;
        ret
    }
}