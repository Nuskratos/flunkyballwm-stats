use crate::calc::accuracy_data::{Accuracy, EnemyAccuracy};
use crate::data::{Game, NamedEntity, Team};
use crate::util::{team_from_player};
use std::collections::HashMap;

pub fn calculate_throwing_accuracy(games: &Vec<Game>) -> Vec<Accuracy> {
    let mut throws = 0;
    let mut hits = 0;
    let mut player_scores: HashMap<NamedEntity, Accuracy> = HashMap::new();
    let mut team_scores: HashMap<NamedEntity, Accuracy> = HashMap::new();

    for game in games {
        for round in &game.rounds {
            let player_accuracy = player_scores
                .entry(round.thrower.named_entity.to_owned())
                .or_insert(Accuracy::default(round.thrower.named_entity.to_owned()));
            let team_accuracy = team_scores
                .entry(
                    team_from_player(round.thrower.id(), game)
                        .named_entity
                        .to_owned(),
                )
                .or_insert(Accuracy::default(
                    team_from_player(round.thrower.id(), game)
                        .named_entity
                        .to_owned(),
                ));
            throws = throws + 1;
            player_accuracy.throws += 1;
            team_accuracy.throws += 1;
            if round.hit {
                hits = hits + 1;
                player_accuracy.hits += 1;
                team_accuracy.hits += 1;
            }
        }
    }

    let mut result_vec: Vec<Accuracy> = Vec::new();
    for score in team_scores {
        result_vec.push(score.1);
    }
    for score in player_scores {
        result_vec.push(score.1);
    }
    result_vec.push(Accuracy {
        throws,
        hits,
        named_entity: NamedEntity {
            name: "Average",
            alias: "Average",
            id: 999,
        },
    });
    result_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_vec.reverse();
    result_vec
}

pub fn calc_enemy_accuracy(games: &Vec<Game>) -> EnemyAccuracy {
    let mut enemy_accuracy: HashMap<&Team, Accuracy> = HashMap::new();
    for game in games {
        let first_enemy_stats = team_from_player(game.rounds.first().unwrap().thrower.id(), game);
        let second_enemy_stats = if &game.left_team == first_enemy_stats {
            &game.right_team
        } else {
            &game.left_team
        };
        for (ix, round) in game.rounds.iter().enumerate() {
            let passive_team = if ix % 2 == 0 {
                second_enemy_stats
            } else {
                first_enemy_stats
            };
            enemy_accuracy
                .entry(passive_team)
                .and_modify(|x| x.add_throw(round.hit))
                .or_insert(Accuracy {
                    named_entity: passive_team.named_entity.to_owned(),
                    hits: if round.hit { 1 } else { 0 },
                    throws: 1,
                });
        }
    }
    let mut acc_vec: Vec<Accuracy> = Vec::new();
    for team in enemy_accuracy {
        acc_vec.push(team.1);
    }
    acc_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    EnemyAccuracy {
        accuracies: acc_vec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2};
    use crate::util::test::{game_2nd_finish, game_2nd_finish_enemy_miss};
    use approx::assert_relative_eq;
    #[test]
    fn test_throwing_accuracy() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM2, TEST_TEAM1),
        ]; // t1_1 2/3 t1_2 2/2
           // t2_1 2/3 t2_2 1/1
        let data = calculate_throwing_accuracy(&games);
        let t1_1 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM1.member_1.name())
            .unwrap();
        let t1_2 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM1.member_2.name())
            .unwrap();
        let t2_1 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM2.member_1.name())
            .unwrap();
        let t2_2 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM2.member_2.name())
            .unwrap();
        let team_1 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM1.name())
            .unwrap();
        let team_2 = data
            .iter()
            .find(|x| x.named_entity.name == TEST_TEAM2.name())
            .unwrap();

        assert_relative_eq!(t1_1.percentage(), 2f32 / 3f32 * 100.0);
        assert_relative_eq!(t1_2.percentage(), 100.0);
        assert_relative_eq!(t2_1.percentage(), 2f32 / 3f32 * 100.0);
        assert_relative_eq!(t2_2.percentage(), 100.0);

        assert_relative_eq!(team_1.percentage(), 80.0);
        assert_relative_eq!(team_2.percentage(), 75.0);
    }

    #[test]
    fn test_enemy_accuracy() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM2, TEST_TEAM1),
        ]; // t1_1 2/3 t1_2 2/2
        // t2_1 2/3 t2_2 1/1
        let data = calc_enemy_accuracy(&games);
        let team1 = data.accuracies.iter().find(|x| x.named_entity.name == TEST_TEAM1.name()).unwrap();
        let team2 = data.accuracies.iter().find(|x| x.named_entity.name == TEST_TEAM2.name()).unwrap();

        assert_relative_eq!(team1.percentage(), 75.0);
        assert_relative_eq!(team2.percentage(), 80.0);
    }
}
