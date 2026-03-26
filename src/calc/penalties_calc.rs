use std::collections::HashMap;
use crate::calc::penalties_data::{Penalties, PenaltiesStatistics, PlayerPenalties, TeamPenalties};
use crate::data::{Game, Team, TeamMember};
use crate::data::AdditionalType::{STRAFBIER, STRAFSCHLUCK};
use crate::util::{team_from_player, team_id_from_player};

pub fn calculate_amount_of_penalties(games: &Vec<Game>) -> PenaltiesStatistics {
    let mut team_map: HashMap<Team, Penalties> = HashMap::new(); // Implementation has a map though it might not need it anymore after switching (u32) to team&player
    let mut player_map: HashMap<TeamMember, Penalties> = HashMap::new();
    for game in games {
        team_map.entry(game.left_team.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        team_map.entry(game.right_team.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        player_map.entry(game.left_1.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        player_map.entry(game.left_2.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        player_map.entry(game.right_1.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        player_map.entry(game.right_2.to_owned()).and_modify(|x| x.games += 1).or_insert(Default::default());
        for round in &game.rounds {
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => {
                        team_map.entry(team_from_player(add.source.id, game).to_owned()).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                        player_map.entry(add.source.to_owned()).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                    }
                    STRAFBIER => {
                        team_map.entry(team_from_player(add.source.id, game).to_owned()).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                        player_map.entry(add.source.to_owned()).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                    }
                    _ => {}
                }
            }
        }
    }
    let mut team_vec: Vec<TeamPenalties> = Vec::new();
    for team in team_map {
        team_vec.push(TeamPenalties{team: team.0, stats:team.1});
    }
    team_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    let mut player_vec: Vec<PlayerPenalties> = Vec::new();
    for player in player_map {
        player_vec.push(PlayerPenalties{player:player.0, stats:player.1});
    }
    player_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    PenaltiesStatistics{teams: team_vec, players: player_vec}
}
