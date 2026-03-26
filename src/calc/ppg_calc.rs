use std::collections::HashMap;
use crate::calc::ppg_data::{PlayerPpg, PpgHolder, PpgStatistics, TeamPpg};
use crate::data::AdditionalType::FINISHED;
use crate::data::{ARC, Game, Team, TeamMember};
use crate::util::player_in_team;

pub fn calculate_amount_of_points_per_game(games: &Vec<Game>) -> PpgStatistics {
    let mut team_map: HashMap<Team, PpgHolder> = HashMap::new();
    let mut player_map: HashMap<TeamMember, PpgHolder> = HashMap::new();
    for game in games {
        team_map.entry(game.left_team.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.left_team.to_owned()).and_modify(|x| x.points += game.result.points_left);
        team_map.entry(game.right_team.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.right_team.to_owned()).and_modify(|x| x.points += game.result.points_right);
        player_map.entry(game.left_1.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.left_2.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_1.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_2.to_owned()).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        let mut round_left_finished = 0;
        let mut round_right_finished = 0;
        let mut left_already_finished = false;
        let mut right_already_finished = false;
        let mut points_vec: Vec<u32> = vec![7, 5, 3];
        for arc in game.additionals_vec() {
            if arc.additional.kind == FINISHED {
                player_map.entry(arc.additional.source.to_owned()).and_modify(|x| x.points += points_vec.first().unwrap());
                points_vec.remove(0);
                // adding points for winning (2 if in the same round as first, so they have the same, 1 to each if in a later round)
                if player_in_team(arc.additional.source.id, &game.left_team) {
                    update_player_map(&mut player_map, round_left_finished, left_already_finished, game, true, &arc);
                } else { // player in right
                    update_player_map(&mut player_map, round_right_finished, right_already_finished, game, false, &arc);
                }
                // Storing the completed round info to know when the next one finishes
                if player_in_team(arc.additional.source.id, &game.left_team) {
                    left_already_finished = true;
                    round_left_finished = arc.round_nr;
                } else {
                    right_already_finished = true;
                    round_right_finished = arc.round_nr;
                }
            }
        }
    }
    let mut team_vec: Vec<TeamPpg> = Vec::new();
    for team in team_map {
        team_vec.push(TeamPpg{team:team.0, stats: team.1});
    }
    team_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    let mut player_vec: Vec<PlayerPpg> = Vec::new();
    for player in player_map {
        player_vec.push(PlayerPpg{player:player.0, stats: player.1});
    }
    player_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    PpgStatistics{teams: team_vec, players: player_vec}
}

fn update_player_map(map: &mut HashMap<TeamMember, PpgHolder>, round_finished: u32, partner_finished: bool, game: &Game, left_team: bool, arc: &ARC) {
    if partner_finished {
        if arc.round_nr == round_finished {
            map.entry(arc.additional.source.to_owned()).and_modify(|x| x.points += 2);
        } else {
            if left_team {
                map.entry(game.left_1.to_owned()).and_modify(|x| x.points += 1);
                map.entry(game.left_2.to_owned()).and_modify(|x| x.points += 1);
            } else {
                map.entry(game.right_1.to_owned()).and_modify(|x| x.points += 1);
                map.entry(game.right_2.to_owned()).and_modify(|x| x.points += 1);
            }
        }
    }
}
