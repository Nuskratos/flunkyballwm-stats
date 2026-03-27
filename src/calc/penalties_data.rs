use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::calc::calculation::{open_writer, percentage};
use crate::data::{Team, TeamMember};
use crate::util::print_line_break;
use crate::team_player_data::NAME_WIDTH;

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

pub struct PlayerPenalties{
    pub player: TeamMember,
    pub stats: Penalties
}
pub struct TeamPenalties{
    pub team: Team,
    pub stats: Penalties
}

pub struct PenaltiesStatistics{
    pub teams: Vec<TeamPenalties>,
    pub players: Vec<PlayerPenalties>
}

impl PenaltiesStatistics{
    pub fn print(&self){
        let width = 12;
        let total_line_width = 91;
        println!("Penalties:");
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Name", "Strafschluck", "Strafbeer", "SpG", "BpG");
        print_line_break(total_line_width);
        for team in &self.teams {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} | {:>width$.2} |", team.team.name(), team.stats.schlucke, team.stats.beers, team.stats.spg(), team.stats.bpg());
        }
        print_line_break(total_line_width);
        for player in &self.players {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} | {:>width$.2} |", player.player.name(), player.stats.schlucke, player.stats.beers, player.stats.spg(), player.stats.bpg());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix: &String, date : &String){
        let (mut writer, file_exists) = open_writer(date.to_string()+"penalties.csv");
        if !file_exists{
            writer.write_record(&["HiddenPrefix", "Team or Player", "Name", "Strafschluck", "Strafbeer", "SpG", "BpG"]);
        }
        for team in &self.teams{
            writer.write_record(&[file_prefix, "Team", team.team.name(), &team.stats.schlucke.to_string(), &team.stats.beers.to_string(), &format!("{:.2}",team.stats.spg()), &format!("{:.2}",team.stats.bpg())]);
        }
        for player in &self.players{
            writer.write_record(&[file_prefix, "Player", player.player.name(), &player.stats.schlucke.to_string(), &player.stats.beers.to_string(), &format!("{:.2}",player.stats.spg()), &format!("{:.2}",player.stats.bpg())]);
        }
    }
}