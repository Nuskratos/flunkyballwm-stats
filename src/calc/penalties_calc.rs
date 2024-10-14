use std::collections::HashMap;
use crate::calc::penalties_data::Penalties;
use crate::data::{Game, Team, TeamMember};
use crate::data::AdditionalType::{STRAFBIER, STRAFSCHLUCK};
use crate::util::team_id_from_player;

pub fn calculate_amount_of_penalties(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) -> (Vec<(u32, Penalties)>, Vec<(u32, Penalties)>) {
    let mut team_map: HashMap<u32, Penalties> = HashMap::new();
    let mut player_map: HashMap<u32, Penalties> = HashMap::new();
    // Fill maps in case someone got no penalty
    for team in teams {
        team_map.insert(team.id, Default::default());
    }
    for player in players {
        player_map.insert(player.id, Default::default());
    }
    for game in games {
        team_map.entry(game.left_team.id).and_modify(|x| x.games += 1);
        team_map.entry(game.right_team.id).and_modify(|x| x.games += 1);
        player_map.entry(game.left_1.id).and_modify(|x| x.games += 1);
        player_map.entry(game.left_2.id).and_modify(|x| x.games += 1);
        player_map.entry(game.right_1.id).and_modify(|x| x.games += 1);
        player_map.entry(game.right_2.id).and_modify(|x| x.games += 1);
        for round in &game.rounds {
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => {
                        team_map.entry(team_id_from_player(add.source.id, game)).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                        player_map.entry(add.source.id).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                    }
                    STRAFBIER => {
                        team_map.entry(team_id_from_player(add.source.id, game)).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                        player_map.entry(add.source.id).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                    }
                    _ => {}
                }
            }
        }
    }
    let mut team_vec: Vec<(u32, Penalties)> = Vec::new();
    for team in team_map {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, Penalties)> = Vec::new();
    for player in player_map {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    (team_vec, player_vec)
}
