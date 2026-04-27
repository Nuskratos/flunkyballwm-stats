use std::collections::{HashMap, HashSet};
use crate::calc::accuracy_after_running_data::{AccuracyAfterRunningData, EntityAccuracy};
use crate::calc::accuracy_data::Accuracy;
use crate::data::{Game, NamedEntity, Round};
use crate::team_player_data::AVERAGE_ENTITY;
use crate::util::team_from_player;

pub fn calculate_accuracy_after_running(games: &Vec<Game>) -> AccuracyAfterRunningData{
    let mut ran_map: HashMap<NamedEntity, Accuracy> = HashMap::new();
    let mut didnt_ran_map: HashMap<NamedEntity, Accuracy> = HashMap::new();
    for game in games{
        if game.special_first_throw.is_some() {
            add_to_maps(&game.special_first_throw.to_owned().unwrap(), &mut ran_map, &mut didnt_ran_map, &None, game);
        }
        for (i, round) in game.rounds.iter().enumerate(){
            if i == 0{
                if game.special_first_throw.is_some() {
                    add_to_maps(round, &mut ran_map, &mut didnt_ran_map, &game.special_first_throw, game);
                }else{
                    add_to_maps(round, &mut ran_map, &mut didnt_ran_map, &None, game);
                }
            }else{
                let prev_round = game.rounds.get(i-1).unwrap().clone();
                add_to_maps(round, &mut ran_map, &mut didnt_ran_map, &Some(prev_round), game);
            }
        }
    }
    /*
    // check effect if only at least 5 throws in each category is there
    let mut list_removeable_entities: Vec<NamedEntity> = vec![];
    for (entity, acc) in ran_map.iter_mut(){
        if acc.throws < 5{
            list_removeable_entities.push(entity.clone());
        }
    }
    for (entity, acc) in didnt_ran_map.iter_mut(){
        if acc.throws < 5{
            list_removeable_entities.push(entity.clone());
        }
    }
    for entity in list_removeable_entities.iter(){
        ran_map.remove(entity);
        didnt_ran_map.remove(entity);
    }*/


    let average = EntityAccuracy {
        running: average_from_map(&ran_map),
        without_running: average_from_map(&didnt_ran_map),
    };
    let all_keys: HashSet<_> = ran_map.keys().chain(didnt_ran_map.keys()).collect();

    // 2. Map them to the
    let mut entity_accuracies: Vec<EntityAccuracy> = all_keys
        .into_iter()
        .map(|entity| EntityAccuracy {
            running: ran_map.get(entity).cloned().unwrap_or(Accuracy::new(entity.clone())),
            without_running: didnt_ran_map.get(entity).cloned().unwrap_or(Accuracy::new(entity.clone())),
        })
        .collect();
    entity_accuracies.sort_by(|a, b| {
        b.difference()
            .partial_cmp(&a.difference())
            .unwrap()
    });

    AccuracyAfterRunningData{entities: entity_accuracies, average}
}
fn add_to_maps(round: &Round, ran_map: &mut HashMap<NamedEntity, Accuracy>, didnt_ran_map: &mut HashMap<NamedEntity, Accuracy>, prev_round: &Option<Round>, game :&Game) {
    let thrower = round.thrower.clone().named_entity;
    let team = team_from_player(thrower.id, game).named_entity;
    if prev_round.is_some() && prev_round.clone().unwrap().runner.id() == thrower.id{
        ran_map.entry(thrower).or_insert(Accuracy::new(thrower)).add_throw(round.hit);
        ran_map.entry(team).or_insert(Accuracy::new(team)).add_throw(round.hit);
    }else{
        didnt_ran_map.entry(thrower).or_insert(Accuracy::new(thrower)).add_throw(round.hit);
        didnt_ran_map.entry(team).or_insert(Accuracy::new(team)).add_throw(round.hit);
    }
}
fn average_from_map(map: &HashMap<NamedEntity, Accuracy>) -> Accuracy{
    let mut acc = Accuracy::new(AVERAGE_ENTITY);
    for (_, acc_per_player) in map.iter(){
        acc.throws += acc_per_player.throws;
        acc.hits += acc_per_player.hits;
    }
    acc.hits = acc.hits / 2;
    acc.throws = acc.throws /2;
    acc
}

#[cfg(test)]

mod test{
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{convert_first_throw_games, game_2nd_finish, game_2nd_finish_enemy_miss, game_5th_finish_strafbeer, game_finished_after_everyone_missed_first};
    use super::*;

    #[test]
    fn test_running_accuracy() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM1),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM1),
        ];

        let data = calculate_accuracy_after_running(&games);

        assert_eq!(data.average.running.throws, 2);
        assert_eq!(data.average.running.hits, 1);
        assert_eq!(data.average.without_running.throws, 4);
        assert_eq!(data.average.without_running.hits, 4);
    }

    #[test]
    fn test_running_accuracy_special_throw() {
        let mut games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM1),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM1),
        ];
        convert_first_throw_games(&mut games);
        let data = calculate_accuracy_after_running(&games);

        assert_eq!(data.average.running.throws, 2);
        assert_eq!(data.average.running.hits, 1);
        assert_eq!(data.average.without_running.throws, 4);
        assert_eq!(data.average.without_running.hits, 4);
    }

    #[test]
    fn test_complex_running_accuracy(){
        let mut games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2),// running: 1/1 non-running: 2/2
        game_finished_after_everyone_missed_first(TEST_TEAM1, TEST_TEAM3), // running: 1/3 non-running: 2/4
        game_2nd_finish_enemy_miss(TEST_TEAM3, TEST_TEAM2),// running: 0/1 non-running: 2/2
        game_2nd_finish(TEST_TEAM2, TEST_TEAM1)]; // running: 1/1 non-running: 2/2
        let data = calculate_accuracy_after_running(&games);
        let t1_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.member_1.named_entity).unwrap();
        let t1_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.member_2.named_entity).unwrap();
        let t2_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.member_1.named_entity).unwrap();
        let t2_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.member_2.named_entity).unwrap();
        let t3_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.member_1.named_entity).unwrap();
        let t3_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.member_2.named_entity).unwrap();
        let team1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.named_entity).unwrap();
        let team2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.named_entity).unwrap();
        let team3 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.named_entity).unwrap();

        assert_eq!(data.average.running.throws, 6);
        assert_eq!(data.average.running.hits, 3);
        assert_eq!(data.average.without_running.throws, 10);
        assert_eq!(data.average.without_running.hits, 8);

        assert_eq!(t1_1.running.throws, 1);
        assert_eq!(t1_1.running.hits, 1);
        assert_eq!(t1_1.without_running.throws, 3);
        assert_eq!(t1_1.without_running.hits, 2);

        assert_eq!(t1_2.running.throws, 0);
        assert_eq!(t1_2.running.hits, 0);
        assert_eq!(t1_2.without_running.throws, 3);
        assert_eq!(t1_2.without_running.hits, 2);

        assert_eq!(t2_1.running.throws, 2);
        assert_eq!(t2_1.running.hits, 1);
        assert_eq!(t2_1.without_running.throws, 1);
        assert_eq!(t2_1.without_running.hits, 1);

        assert_eq!(t2_2.running.throws, 0);
        assert_eq!(t2_2.running.hits, 0);
        assert_eq!(t2_2.without_running.throws, 1);
        assert_eq!(t2_2.without_running.hits, 1);

        assert_eq!(t3_1.running.throws, 2);
        assert_eq!(t3_1.running.hits, 1);
        assert_eq!(t3_1.without_running.throws, 1);
        assert_eq!(t3_1.without_running.hits, 1);

        assert_eq!(t3_2.running.throws, 1);
        assert_eq!(t3_2.running.hits, 0);
        assert_eq!(t3_2.without_running.throws, 1);
        assert_eq!(t3_2.without_running.hits, 1);
    }

    #[test]
    fn test_complex_running_accuracy_special_throw(){
        let mut games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2),// running: 1/1 non-running: 2/2
                             game_finished_after_everyone_missed_first(TEST_TEAM1, TEST_TEAM3), // running: 1/3 non-running: 2/4
                             game_2nd_finish_enemy_miss(TEST_TEAM3, TEST_TEAM2),// running: 0/1 non-running: 2/2
                             game_2nd_finish(TEST_TEAM2, TEST_TEAM1)]; // running: 1/1 non-running: 2/2
        convert_first_throw_games(&mut games);
        let data = calculate_accuracy_after_running(&games);
        let t1_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.member_1.named_entity).unwrap();
        let t1_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.member_2.named_entity).unwrap();
        let t2_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.member_1.named_entity).unwrap();
        let t2_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.member_2.named_entity).unwrap();
        let t3_1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.member_1.named_entity).unwrap();
        let t3_2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.member_2.named_entity).unwrap();
        let team1 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM1.named_entity).unwrap();
        let team2 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM2.named_entity).unwrap();
        let team3 = data.entities.iter().find(|e| e.running.named_entity == TEST_TEAM3.named_entity).unwrap();

        assert_eq!(data.average.running.throws, 6);
        assert_eq!(data.average.running.hits, 3);
        assert_eq!(data.average.without_running.throws, 10);
        assert_eq!(data.average.without_running.hits, 8);

        assert_eq!(t1_1.running.throws, 1);
        assert_eq!(t1_1.running.hits, 1);
        assert_eq!(t1_1.without_running.throws, 3);
        assert_eq!(t1_1.without_running.hits, 2);

        assert_eq!(t1_2.running.throws, 0);
        assert_eq!(t1_2.running.hits, 0);
        assert_eq!(t1_2.without_running.throws, 3);
        assert_eq!(t1_2.without_running.hits, 2);

        assert_eq!(t2_1.running.throws, 2);
        assert_eq!(t2_1.running.hits, 1);
        assert_eq!(t2_1.without_running.throws, 1);
        assert_eq!(t2_1.without_running.hits, 1);

        assert_eq!(t2_2.running.throws, 0);
        assert_eq!(t2_2.running.hits, 0);
        assert_eq!(t2_2.without_running.throws, 1);
        assert_eq!(t2_2.without_running.hits, 1);

        assert_eq!(t3_1.running.throws, 2);
        assert_eq!(t3_1.running.hits, 1);
        assert_eq!(t3_1.without_running.throws, 1);
        assert_eq!(t3_1.without_running.hits, 1);

        assert_eq!(t3_2.running.throws, 1);
        assert_eq!(t3_2.running.hits, 0);
        assert_eq!(t3_2.without_running.throws, 1);
        assert_eq!(t3_2.without_running.hits, 1);
    }
}