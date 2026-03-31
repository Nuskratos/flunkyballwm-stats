use std::cmp::Ordering;
use crate::calc::calculation::{wrong_way_average};
use crate::data::{NamedEntity, TeamMember};
use crate::util::{open_writer, print_line_break, OpenedWriter};
use crate::team_player_data::NAME_WIDTH;

pub struct ThrowData {
    pub team: Vec<ThrowsPerGame>,
    pub player: Vec<ThrowsPerGame>,
    pub average: ThrowsPerGame
}

impl ThrowData{

    pub fn print(&self){
        println!("Average throws per game:");
        let width = 10;
        let total_width = 70;
        println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Team", "Games", "Throws", "Average");
        print_line_break(total_width);
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", "Total", self.average.games, self.average.throws, self.average.average());
        print_line_break(total_width);
        for team_throws_per_game in &self.team {
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team_throws_per_game.named_entity.name, team_throws_per_game.games, team_throws_per_game.throws, wrong_way_average(team_throws_per_game.games, team_throws_per_game.throws));
        }
        print_line_break(total_width);
        for player_throws_per_game in &self.player{
            println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", player_throws_per_game.named_entity.name, player_throws_per_game.games, player_throws_per_game.throws, wrong_way_average(player_throws_per_game.games, player_throws_per_game.throws));
        }
        println!();
    }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let filesufix= "throws_per_game.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Team or Player", "Name", "Games", "Throws", "Average"]);
        }
        opened_writer.writer.write_record(&[file_prefix, "Average", &self.average.named_entity.name_or_alias(write_alias), &self.average.games.to_string(), &self.average.throws.to_string(), &self.average.average_str()]);
        for team in &self.team{
            opened_writer.writer.write_record(&[file_prefix, "Team", &team.named_entity.name_or_alias(write_alias), &team.games.to_string(), &team.throws.to_string(), &team.average_str()]);
        }
        for player in &self.player{
            opened_writer.writer.write_record(&[file_prefix, "Player", &player.named_entity.name_or_alias(write_alias), &player.games.to_string(), &player.throws.to_string(), &player.average_str()]);
        }
    }
}

pub struct ThrowsPerGame {
    pub named_entity: NamedEntity,
    pub throws: u32,
    pub games: u32,
}

impl ThrowsPerGame {
    pub fn add_throws(&mut self, throws: u32) {
        self.throws += throws;
        self.games += 1;
    }
    pub fn new(entity:NamedEntity) -> ThrowsPerGame {
        ThrowsPerGame {
            throws: 0,
            games: 1,
            named_entity:entity
        }
    }
    pub fn average(&self) -> f32 {
        self.throws as f32 / self.games as f32
    }
    pub fn average_str(&self)->String{
        format!("{:.3}", self.average())
    }
    pub fn custom_cmp(&self, other: &Self) -> Option<Ordering> {
        self.average().partial_cmp(&other.average())
    }
}