use std::collections::HashMap;
use crate::calc::ppg_data::PpgHolder;
use crate::data::AdditionalType::FINISHED;
use crate::data::{ARC, Game};
use crate::util::player_in_team;

pub fn calculate_amount_of_points_per_game(games: &Vec<Game>) -> (Vec<(u32, PpgHolder)>, Vec<(u32, PpgHolder)>) {
    let mut team_map: HashMap<u32, PpgHolder> = HashMap::new();
    let mut player_map: HashMap<u32, PpgHolder> = HashMap::new();
    for game in games {
        team_map.entry(game.left_team.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.left_team.id).and_modify(|x| x.points += game.result.points_left);
        team_map.entry(game.right_team.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.right_team.id).and_modify(|x| x.points += game.result.points_right);
        player_map.entry(game.left_1.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.left_2.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_1.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_2.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        let mut round_left_finished = 0;
        let mut round_right_finished = 0;
        let mut left_already_finished = false;
        let mut right_already_finished = false;
        let mut points_vec: Vec<u32> = vec![7, 5, 3];
        for arc in game.additionals_vec() {
            if arc.additional.kind == FINISHED {
                player_map.entry(arc.additional.source.id).and_modify(|x| x.points += points_vec.first().unwrap());
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
    let mut team_vec: Vec<(u32, PpgHolder)> = Vec::new();
    for team in team_map {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, PpgHolder)> = Vec::new();
    for player in player_map {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    (team_vec, player_vec)
}

fn update_player_map(map: &mut HashMap<u32, PpgHolder>, round_finished: u32, partner_finished: bool, game: &Game, left_team: bool, arc: &ARC) {
    if partner_finished {
        if arc.round_nr == round_finished {
            map.entry(arc.additional.source.id).and_modify(|x| x.points += 2);
        } else {
            if left_team {
                map.entry(game.left_1.id).and_modify(|x| x.points += 1);
                map.entry(game.left_2.id).and_modify(|x| x.points += 1);
            } else {
                map.entry(game.right_1.id).and_modify(|x| x.points += 1);
                map.entry(game.right_2.id).and_modify(|x| x.points += 1);
            }
        }
    }
}
