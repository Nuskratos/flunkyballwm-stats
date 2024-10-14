use std::collections::HashMap;
use crate::calc::chain_data::ChainInformation;
use crate::data::{Game, Team, TeamMember};
use crate::util::team_from_player;

pub fn calculate_hit_and_miss_chains_team_player(games:&Vec<Game>, teams:&Vec<Team>) -> (Vec<(u32, ChainInformation)>, Vec<(u32, ChainInformation)>){
    let mut team_chain: HashMap<&Team, ChainInformation> = HashMap::new();
    let mut player_chain: HashMap<&TeamMember, ChainInformation> = HashMap::new();
    for game in games {
        for round in &game.rounds {
            let team = team_from_player(round.thrower.id, game);
            team_chain.entry(team).and_modify(|x| x.throw(round.hit)).or_insert(ChainInformation::create(round.hit));
            player_chain.entry(&round.thrower).and_modify(|x| x.throw(round.hit)).or_insert(ChainInformation::create(round.hit));
        }
    }
    let mut team_vec: Vec<(u32, ChainInformation)> = Vec::new();
    for team in team_chain {
        team_vec.push((team.0.id, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, ChainInformation)> = Vec::new();
    for player in player_chain {
        player_vec.push((player.0.id, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    (team_vec, player_vec)
}