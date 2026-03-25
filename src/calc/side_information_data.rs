use std::fs::File;
use csv::Writer;

pub struct SideSplit {
    pub left: SideInformation,
    pub right: SideInformation,
}

impl SideSplit {
    pub fn serialize(&self, writer: &mut Writer<File>, file_prefix: &String){
        self.left.serialize(writer, file_prefix, "left");
        self.right.serialize(writer, file_prefix, "right");
    }

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

impl SideInformation {
    pub fn serialize(&self, writer: &mut Writer<File>, file_prefix: &String, side_string : &str){
        let percentage = self.hits as f32 / self.throws as f32;
        let perc_str = format!("{:.2}%", percentage);
        writer.write_record(&[file_prefix, side_string, &self.wins.to_string(),&self.points.to_string(),
            &self.hits.to_string(), &self.throws.to_string(), &perc_str,&self.schluck.to_string(), &self.beer.to_string()]);
    }
}
