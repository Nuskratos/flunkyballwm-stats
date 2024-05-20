pub struct SideSplit {
    pub left: SideInformation,
    pub right: SideInformation,
}

#[derive(Default)]
pub struct SideInformation {
    pub wins: u32,
    pub points: u32,
    pub hits: u32,
    pub throws: u32,
    pub schluck: u32,
    pub beer: u32,
}
