use std::collections::HashMap;
use crate::calc::calculation::percentage;
use crate::calc::first_throw_data::{FirstThrows, FirstTeamThrowsStatistics, NamedThrows, FirstThrowStatistic};
use crate::data::{Game, NamedEntity, Team};
use crate::util::{team_from_player, team_id_from_player, team_name_from_id};

pub fn calc_team_first_throws(games: &Vec<Game>) -> FirstTeamThrowsStatistics {
    // Times going first, times won going first, times going second, times won going second
    let mut first_throws: HashMap<NamedEntity, FirstThrows> = HashMap::new();
    for game in games {
        let team_id_going_first = team_from_player(game.rounds.first().unwrap().thrower.id(), game).named_entity.to_owned();
        let throws = first_throws.entry(team_id_going_first.to_owned()).or_insert(FirstThrows::default());
        throws.threw_first += 1;
        if team_id_going_first == game.winning_team().named_entity {
            throws.won_first += 1;
        } else {}
    }
    for game in games { // duplicate because of 2nd mutable borrow in first_throws.entry TODO make prettier
        let Team_going_second = team_from_player(game.rounds.first().unwrap().runner.id(), game).named_entity.to_owned();
        let throws = first_throws.entry(Team_going_second.to_owned()).or_insert(FirstThrows::default());
        throws.threw_second += 1;
        if Team_going_second == game.winning_team().named_entity {
            throws.won_second += 1;
        }
    }

    let mut result_team_vec: Vec<NamedThrows> = Vec::new();
    for team_info in first_throws {
        result_team_vec.push(NamedThrows{named_entity:team_info.0, throw_data:team_info.1});
    }
    result_team_vec.sort_by(|a, b| a.throw_data.threw_first.cmp(&b.throw_data.threw_first));
    result_team_vec.reverse();
    FirstTeamThrowsStatistics {teams:result_team_vec}
}

pub fn calc_general_first_throw(games:&Vec<Game>)-> FirstThrowStatistic{

    let mut first_throw_win = 0;
    let mut first_hit:u32 = 0;
    let mut first_hit_win_amount = 0;
    for game in games {
        let mut first_hit_bool = false;
        if game.rounds.first().unwrap().hit {
            first_hit += 1;
            first_hit_bool = true;
        }
        let mut winning_ids = (0, 0);
        if game.result.points_left > game.result.points_right {
            winning_ids = (game.left_1.id(), game.left_2.id());
        } else {
            winning_ids = (game.right_1.id(), game.right_2.id());
        }
        let thrower_id = game.rounds.first().unwrap().thrower.id();
        if thrower_id == winning_ids.0 || thrower_id == winning_ids.1 {
            first_throw_win += 1;
            if first_hit_bool {
                first_hit_win_amount += 1;
            }
        }
    }
    let first_miss = games.len() as u32- first_hit;
    let first_miss_win_amount = first_throw_win - first_hit_win_amount;
    FirstThrowStatistic{games:games.len() as u32, first_throw_win, first_hit,first_hit_win_amount,first_miss, first_miss_win_amount,}
}