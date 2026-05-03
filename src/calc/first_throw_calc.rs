use crate::calc::calculation::percentage;
use crate::calc::first_throw_data::{FirstEntityThrowsStatistics, FirstThrowStatistic, FirstThrows, NamedThrows};
use crate::data::{Game, NamedEntity, Team};
use crate::util::{team_from_player, team_id_from_player, team_name_from_id, teammate_from_player};
use std::collections::HashMap;

pub fn calc_team_first_throws(games: &Vec<Game>) -> FirstEntityThrowsStatistics {
    let mut first_throws: HashMap<NamedEntity, FirstThrows> = HashMap::new();
    for game in games {
        let team_going_first = team_from_player(game.first_throw().thrower.id(), game).named_entity.to_owned();
        let first_won_game = team_going_first == game.winning_team().named_entity;
        add_entity_throws_to_map(&mut first_throws, team_going_first, first_won_game, true);
        add_entity_throws_to_map(&mut first_throws, game.first_throw().thrower.named_entity, first_won_game, true);
        add_entity_throws_to_map(&mut first_throws, teammate_from_player(game.first_throw().thrower.id(), game).unwrap().named_entity, first_won_game, true);

        //2nd team
        let team_going_second = team_from_player(game.first_throw().runner.id(), game).named_entity.to_owned();
        let second_won_game = team_going_second == game.winning_team().named_entity;
        add_entity_throws_to_map(&mut first_throws, team_going_second, second_won_game, false);
        add_entity_throws_to_map(&mut first_throws, game.first_throw().runner.named_entity, second_won_game, false);
        add_entity_throws_to_map(&mut first_throws, teammate_from_player(game.first_throw().runner.id(), game).unwrap().named_entity, second_won_game, false);
    }

    let mut result_vec: Vec<NamedThrows> = Vec::new();
    for entity_info in first_throws {
        result_vec.push(NamedThrows { named_entity: entity_info.0, throw_data: entity_info.1 });
    }
    result_vec.sort_by(|a, b| a.throw_data.threw_first.cmp(&b.throw_data.threw_first));
    result_vec.reverse();
    FirstEntityThrowsStatistics { teams: result_vec }
}

fn add_entity_throws_to_map(map: &mut HashMap<NamedEntity, FirstThrows>, named_entity: NamedEntity, won_game: bool, threw_first:bool) {
    let throws = map.entry(named_entity.to_owned()).or_insert(FirstThrows::default());
    if(threw_first){
        throws.threw_first += 1;
        if won_game {
            throws.won_first += 1;
        }
    }else{
        throws.threw_second += 1;
        if won_game {
            throws.won_second += 1;
        }
    }
}

pub fn calc_general_first_throw(games: &Vec<Game>) -> FirstThrowStatistic {
    let mut first_throw_win = 0;
    let mut first_hit: u32 = 0;
    let mut first_hit_win_amount = 0;
    for game in games {
        let mut first_hit_bool = false;
        if game.first_throw().hit {
            first_hit += 1;
            first_hit_bool = true;
        }
        let mut winning_ids = (0, 0);
        if game.result.points_left > game.result.points_right {
            winning_ids = (game.left_1.id(), game.left_2.id());
        } else {
            winning_ids = (game.right_1.id(), game.right_2.id());
        }
        let thrower_id = game.first_throw().thrower.id();
        if thrower_id == winning_ids.0 || thrower_id == winning_ids.1 {
            first_throw_win += 1;
            if first_hit_bool {
                first_hit_win_amount += 1;
            }
        }
    }
    let first_miss = games.len() as u32 - first_hit;
    let first_miss_win_amount = first_throw_win - first_hit_win_amount;
    FirstThrowStatistic {
        games: games.len() as u32,
        first_throw_win,
        first_hit,
        first_hit_win_amount,
        first_miss,
        first_miss_win_amount,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2};
    use crate::util::test::{
        game_2nd_finish, game_2nd_finish_after_enemy_began, game_2nd_finish_right_began,
    };
    use approx::assert_relative_eq;
    use crate::util::convert_first_throw_games;

    #[test]
    fn test_general_first_throw() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_after_enemy_began(TEST_TEAM1, TEST_TEAM2),
        ];
        let data = calc_general_first_throw(&games);
        assert_eq!(data.first_hit_win_amount, 2);
        assert_eq!(data.games, 3);
        assert_relative_eq!(data.first_throw_win(), 2f32 / 3f32);
    }

    #[test]
    fn test_specific_first_throw() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM2, TEST_TEAM1),
            game_2nd_finish_after_enemy_began(TEST_TEAM2, TEST_TEAM1),
        ]; // Team1 (first: 2/3, second 0/1)
           // Team2 (first: 1/1, second 1/3)
        let data = calc_team_first_throws(&games);
        let team1 = data.teams.iter().find(|x| x.named_entity.name == TEST_TEAM1.name()).unwrap();
        let team2 = data.teams.iter().find(|x| x.named_entity.name == TEST_TEAM2.name()).unwrap();

        assert_relative_eq!(team1.throw_data.win_perc(), 2f32 / 3f32 * 100.0);
        assert_relative_eq!(team1.throw_data.second_perc(), 0.0);
        assert_relative_eq!(team2.throw_data.win_perc(), 100.0);
        assert_relative_eq!(team2.throw_data.second_perc(), 1f32 / 3f32 * 100.0);
    }
    #[test]
    fn test_specific_first_throw_new() {
        let mut games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM2, TEST_TEAM1),
            game_2nd_finish_after_enemy_began(TEST_TEAM2, TEST_TEAM1),
        ]; // Team1 (first: 2/3, second 0/1)
           // Team2 (first: 1/1, second 1/3)
        convert_first_throw_games(&mut games);
        let data = calc_team_first_throws(&games);
        let team1 = data.teams.iter().find(|x| x.named_entity.name == TEST_TEAM1.name()).unwrap();
        let team2 = data.teams.iter().find(|x| x.named_entity.name == TEST_TEAM2.name()).unwrap();

        assert_relative_eq!(team1.throw_data.win_perc(), 2f32 / 3f32 * 100.0);
        assert_relative_eq!(team1.throw_data.second_perc(), 0.0);
        assert_relative_eq!(team2.throw_data.win_perc(), 100.0);
        assert_relative_eq!(team2.throw_data.second_perc(), 1f32 / 3f32 * 100.0);
    }

    #[test]
    fn test_first_works_new() {
        let mut games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_after_enemy_began(TEST_TEAM1, TEST_TEAM2),
        ];
        convert_first_throw_games(&mut games);
        let data = calc_general_first_throw(&games);
        assert_eq!(data.first_hit_win_amount, 2);
        assert_eq!(data.games, 3);
        assert_relative_eq!(data.first_throw_win(), 2f32 / 3f32);
    }
}
