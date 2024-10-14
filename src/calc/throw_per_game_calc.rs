use std::collections::HashMap;
use crate::calc::throw_per_game_data::{ThrowData, ThrowsPerGame};
use crate::data::{Game, Team};
use crate::util::{team_from_player, team_id_from_player};

pub fn calculate_throws_per_game(games: &Vec<Game>, teams: &Vec<Team>) -> ThrowData {
    let mut total_throws: u32 = 0;
    let mut team_throws: HashMap<u32, ThrowsPerGame> = HashMap::new();
    let mut player_throws: HashMap<u32, ThrowsPerGame> = HashMap::new();
    for game in games {
        add_game_count_to_maps(game, &mut team_throws, &mut player_throws);
        total_throws += game.rounds.len() as u32;
        for round in &game.rounds {
            player_throws.entry(round.thrower.id).and_modify(|x| x.throws += 1);
            team_throws.entry(team_id_from_player(round.thrower.id, game)).and_modify(|x| x.throws +=1);
        }
    }
    let mut team_vec: Vec<(u32, ThrowsPerGame)> = Vec::new();
    for team in team_throws {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, ThrowsPerGame)> = Vec::new();
    for player in player_throws {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    ThrowData { team: team_vec, total_throws, player: player_vec }
}

fn add_game_count_to_maps(game: &Game, team_map: &mut HashMap<u32, ThrowsPerGame>, player_map: &mut HashMap<u32, ThrowsPerGame>) {
    team_map.entry(game.left_team.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
    team_map.entry(game.right_team.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
    player_map.entry(game.left_1.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
    player_map.entry(game.left_2.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
    player_map.entry(game.right_1.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
    player_map.entry(game.right_2.id).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new());
}

#[cfg(test)]
mod test{
    use float_cmp::approx_eq;
    use crate::calc::throw_per_game_calc::calculate_throws_per_game;
    use crate::team_player_data::{TEAM_INVALID, TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_2nd_finish, game_3rd_finish};

    #[test]
    fn basic_games(){
        let games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2), game_3rd_finish(TEST_TEAM3, TEST_TEAM2)];
        let teams = vec![TEST_TEAM1, TEST_TEAM2, TEST_TEAM3];
        let data = calculate_throws_per_game(&games,&teams);
        assert_eq!(data.total_throws, 8);
        let first_avg =data.team.iter().find(|x|x.0 == TEST_TEAM1.id).unwrap().1.average();
        let second_avg =data.team.iter().find(|x|x.0 == TEST_TEAM2.id).unwrap().1.average();
        let third_avg =data.team.iter().find(|x|x.0 == TEST_TEAM3.id).unwrap().1.average();
        assert!(approx_eq!(f32, first_avg, 2.0));
        assert!(approx_eq!(f32, second_avg, 1.5));
        assert!(approx_eq!(f32, third_avg, 3.0));
    }
}