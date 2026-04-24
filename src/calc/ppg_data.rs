use std::cmp::Ordering;
use crate::data::{Team, TeamMember};
use crate::util::{open_writer, print_line_break, OpenedWriter};
use crate::team_player_data::NAME_WIDTH;

pub struct PpgHolder {
    pub points: u32,
    pub games: u32,
}

impl PpgHolder {
    pub fn ppg(&self) -> f32 {
        self.points as f32 / self.games as f32
    }
    pub fn ppg_string(&self)->String{
        format!("{:.2}", self.ppg())
    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = other.ppg().partial_cmp(&self.ppg());
        ord
    }
    pub fn new() -> PpgHolder {
        PpgHolder { points: 0, games: 1 }
    }
}
pub struct TeamPpg {
    pub team: Team,
    pub stats: PpgHolder,
}
pub struct PlayerPpg{
    pub player: TeamMember,
    pub stats:PpgHolder
}

pub struct PpgStatistics{
    pub teams:Vec<TeamPpg>,
    pub players: Vec<PlayerPpg>
}

impl PpgStatistics{
    pub fn print(&self){
        let width = 9;
        let total_line_width = 55;
        println!("Points per Game:");
        println!("While the points per Game for a team are being a solid representation, the PpG for a Person should be taken with a grain of salt!\n\
    It is calculated, that the points are split if both finished in the same round, and that they split the 2 points for a win if they finish in different rounds.\n\
    Being in a Team with a fast drinker will significantly reduce this metric, and throwing accuracy is completely ignored!");
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} |", "Name", "Points", "PpG");
        print_line_break(total_line_width);
        for team in &self.teams {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$.2} |", team.team.name(), team.stats.points, team.stats.ppg());
        }
        print_line_break(total_line_width);
        for player in &self.players {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$.2} |", player.player.name(), player.stats.points, player.stats.ppg());
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let filesufix= "ppg_statistics.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Team or Player", "Name", "Points", "PpG"]);
        }
        for team in &self.teams{
            opened_writer.writer.write_record(&[file_prefix, "Team", &team.team.named_entity.name_or_alias(write_alias), &team.stats.points.to_string(), &team.stats.ppg_string()]);
        }
        for player in &self.players{
            opened_writer. writer.write_record(&[file_prefix, "Player", &player.player.named_entity.name_or_alias(write_alias), &player.stats.points.to_string(), &player.stats.ppg_string()]);
        }
    }
}