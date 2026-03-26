use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::team_player_data::NAME_WIDTH;
use crate::calc::calculation::open_writer;
use crate::data::{Team, TeamMember};
use crate::util::{player_name_from_id, print_line_break};

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

pub struct TeamChainInformation{
    pub team: Team,
    pub stats: ChainInformation
}

pub struct PlayerChainInformation{
    pub player:TeamMember,
    pub stats: ChainInformation
}
pub struct ChainStatistics{
    pub teams : Vec<TeamChainInformation>,
    pub players : Vec<PlayerChainInformation>
}

impl ChainStatistics{

    pub fn print(&self){
        let width = 11;
        let total_line_width = 59;
        println!("Hit and miss chains");
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} |", "Name", "Hit-chain", "Miss-chain");
        print_line_break(total_line_width);
        for team in &self.teams {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", team.team.name, team.stats.total_hit, team.stats.total_miss);
        }
        print_line_break(total_line_width);
        for player in &self.players {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", player.player.name, player.stats.total_hit, player.stats.total_miss);
        }
        println!();

    }
    pub fn serialize(&self, file_prefix: &String, date : &String){
        let (mut writer, file_exists) = open_writer(date.to_string()+"throw_chains.csv");
        if !file_exists{
            writer.write_record(&["HiddenPrefix", "Team or Player", "Name", "Hit-chain", "Miss-chain"]);
        }
        for team in &self.teams{
            writer.write_record(&[file_prefix, "Team", team.team.name, &team.stats.total_hit.to_string(), &team.stats.total_miss.to_string()]);
        }
        for player in &self.players{
            writer.write_record(&[file_prefix, "Player", player.player.name, &player.stats.total_hit.to_string(), &player.stats.total_miss.to_string()]);
        }
    }
}