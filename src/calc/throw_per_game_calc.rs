use std::collections::HashMap;
use crate::calc::accuracy_data::Accuracy;
use crate::calc::throw_per_game_data::{ThrowData, ThrowsPerGame};
use crate::data::{Game, NamedEntity};
use crate::team_player_data::AVERAGE_ENTITY;
use crate::util::{team_from_player, team_id_from_player};

pub fn calculate_throws_per_game(games: &Vec<Game>) -> ThrowData {
    let mut total_throws: u32 = 0;
    let mut team_throws: HashMap<NamedEntity, ThrowsPerGame> = HashMap::new();
    let mut player_throws: HashMap<NamedEntity, ThrowsPerGame> = HashMap::new();
    for game in games {
        add_game_count_to_maps(game, &mut team_throws, &mut player_throws);
        total_throws += game.rounds.len() as u32;
        for round in &game.rounds {
            player_throws.entry(round.thrower.named_entity.to_owned()).and_modify(|x| x.throws += 1);
            team_throws.entry(team_from_player(round.thrower.id(), game).named_entity.to_owned()).and_modify(|x| x.throws +=1);
        }
    }
    let mut team_vec: Vec<ThrowsPerGame> = Vec::new();
    for (_entity, tpg) in team_throws {
        team_vec.push(tpg);
    }
    team_vec.sort_by(|a, b| a.custom_cmp(&b).unwrap());
    let mut player_vec: Vec<ThrowsPerGame> = Vec::new();
    for (_entity, tpg) in player_throws {
        player_vec.push(tpg);
    }
    player_vec.sort_by(|a, b| a.custom_cmp(&b).unwrap());
    ThrowData { team: team_vec, player: player_vec, average:ThrowsPerGame{named_entity:AVERAGE_ENTITY, throws:total_throws,games:games.len() as u32}}
}

fn add_game_count_to_maps(game: &Game, team_map: &mut HashMap<NamedEntity, ThrowsPerGame>, player_map: &mut HashMap<NamedEntity, ThrowsPerGame>) {
    team_map.entry(game.left_team.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.left_team.named_entity.to_owned()));
    team_map.entry(game.right_team.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.right_team.named_entity.to_owned()));
    player_map.entry(game.left_1.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.left_1.named_entity.to_owned()));
    player_map.entry(game.left_2.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.left_2.named_entity.to_owned()));
    player_map.entry(game.right_1.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.right_1.named_entity.to_owned()));
    player_map.entry(game.right_2.named_entity.to_owned()).and_modify(|x|x.games+=1).or_insert(ThrowsPerGame::new(game.right_2.named_entity.to_owned()));
}

#[cfg(test)]
mod test{
    use float_cmp::approx_eq;

    use crate::calc::throw_per_game_calc::calculate_throws_per_game;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_2nd_finish, game_3rd_finish};

    #[test]
    fn basic_games(){
        let games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2), game_3rd_finish(TEST_TEAM3, TEST_TEAM2)];
        let teams = vec![TEST_TEAM1, TEST_TEAM2, TEST_TEAM3];
        let data = calculate_throws_per_game(&games);
        assert_eq!(data.average.throws, 8);
        let first_avg =data.team.iter().find(|x|x.named_entity.name == TEST_TEAM1.name()).unwrap().average();
        let second_avg =data.team.iter().find(|x|x.named_entity.name == TEST_TEAM2.name()).unwrap().average();
        let third_avg =data.team.iter().find(|x|x.named_entity.name == TEST_TEAM3.name()).unwrap().average();
        assert!(approx_eq!(f32, first_avg, 2.0));
        assert!(approx_eq!(f32, second_avg, 1.5));
        assert!(approx_eq!(f32, third_avg, 3.0));
    }
}