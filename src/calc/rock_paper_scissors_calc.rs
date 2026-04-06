use std::collections::HashMap;
use crate::calc::rock_paper_scissors_data::{RockPaperScissorSingleStats, RockPaperScissorStats};
use crate::data;
use crate::data::{Entity, Game};
use crate::util::team_from_player;

pub fn calculate_rock_paper_scissors(games: &Vec<Game>) -> RockPaperScissorStats{
    let mut scores : HashMap<Entity, RockPaperScissorSingleStats> = HashMap::new();
    for game in games {
        let winner:Entity = data::Entity::Player(game.rounds.first().unwrap().thrower);
        let loser:Entity = data::Entity::Player(game.rounds.first().unwrap().runner);
        let winner_team:Entity = data::Entity::Team(team_from_player(winner.named().id, game).to_owned());
        let loser_team:Entity = data::Entity::Team(team_from_player(loser.named().id, game).to_owned());
        scores.entry(winner.to_owned()).or_insert(RockPaperScissorSingleStats::default(&winner)).won(&loser);
        scores.entry(loser.to_owned()).or_insert(RockPaperScissorSingleStats::default(&loser)).lost(&winner);
        scores.entry(winner_team.to_owned()).or_insert(RockPaperScissorSingleStats::default(&winner_team)).won(&loser_team);
        scores.entry(loser_team.to_owned()).or_insert(RockPaperScissorSingleStats::default(&loser_team)).lost(&winner_team);
    }
    let mut data: Vec<_> = scores.into_values().collect();
    data.sort();
    RockPaperScissorStats{data}
}