use std::collections::HashMap;
use crate::calc::first_throw_data::{FirstThrows, FirstThrowsStatistics, NamedThrows};
use crate::data::{Game, NamedEntity, Team};
use crate::util::{team_from_player, team_id_from_player, team_name_from_id};

pub fn calc_team_first_throws(games: &Vec<Game>) ->FirstThrowsStatistics {
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
    FirstThrowsStatistics{teams:result_team_vec}
}