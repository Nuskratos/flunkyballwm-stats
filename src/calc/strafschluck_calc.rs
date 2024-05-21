use std::collections::HashMap;
use crate::calc::strafschluck_data::{StrafschluckCounter, StrafschluckData};
use crate::data::{Additional, Game, Team};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::util::{player_in_team, team_from_player};

pub fn calculate_strafschluck(games: &Vec<Game>, teams: &Vec<Team>) -> StrafschluckData {
    let mut data: StrafschluckData = Default::default();
    for game in games {
        let mut first_counter: StrafschluckCounter = StrafschluckCounter::new();
        let mut second_counter: StrafschluckCounter = StrafschluckCounter::new();
        let first_team = team_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let mut strafbeer_hit: HashMap<u32, u32> = HashMap::new();
        let mut strafbeer_schluck: HashMap<u32, u32> = HashMap::new();
        for (ix, round) in game.rounds.iter().enumerate() {
            if round.hit {
                if ix % 2 == 0 {
                    first_counter.hits += 1;
                } else {
                    second_counter.hits += 1;
                }
            }
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => {
                        store_strafschluck(&add, first_team, &mut first_counter, &mut second_counter);
                    }
                    STRAFBIER => {
                        add_completed_beer(&mut data, &add, first_team, &mut strafbeer_hit, &mut strafbeer_schluck, &first_counter, &second_counter, 1);
                        store_strafbeer_stats(&add, first_team, &mut strafbeer_hit, &mut strafbeer_schluck, &first_counter, &second_counter);
                    }
                    FINISHED => {
                        add_completed_beer(&mut data, &add, first_team, &mut strafbeer_hit, &mut strafbeer_schluck, &first_counter, &second_counter, 0);
                    }
                }
            }
        }
    }
    data
}

fn store_strafschluck(add: &Additional, first_team: &Team, first: &mut StrafschluckCounter, second: &mut StrafschluckCounter) {
    if player_in_team(add.source.id, first_team) {
        second.schluck_drank += 1;
        second.clean = false;
    } else {
        first.schluck_drank += 1;
        first.clean = false;
    }
}

fn store_strafbeer_stats(add: &Additional, first_team: &Team, strafbeer_hit: &mut HashMap<u32, u32>, strafbeer_schluck: &mut HashMap<u32, u32>, first: &StrafschluckCounter, second: &StrafschluckCounter) {
    if player_in_team(add.source.id, first_team) {
        strafbeer_hit.entry(add.source.id).and_modify(|x| *x = first.hits).or_insert(first.hits);
        strafbeer_schluck.entry(add.source.id).and_modify(|x| *x = first.schluck_drank).or_insert(first.schluck_drank);
    } else {
        strafbeer_hit.entry(add.source.id).and_modify(|x| *x = second.hits).or_insert(second.hits);
        strafbeer_schluck.entry(add.source.id).and_modify(|x| *x = second.schluck_drank).or_insert(second.schluck_drank);
    }
}

fn add_completed_beer(data: &mut StrafschluckData, add: &Additional, first_team: &Team, strafbeer_hit: &mut HashMap<u32, u32>, strafbeer_schluck: &mut HashMap<u32, u32>, first: &StrafschluckCounter, second: &StrafschluckCounter, additional_hit_from_strafbeer: u32) {
    if player_in_team(add.source.id, first_team) {
        let (hits_for_this_beer, schlucke_for_this_beer) = hits_and_schlucke_for_this_beer(&strafbeer_hit, first.hits, &strafbeer_schluck, first.schluck_drank, &add.source.id);
        data.add_finished_beer(first.clean, hits_for_this_beer, schlucke_for_this_beer, additional_hit_from_strafbeer);
    } else {
        let (hits_for_this_beer, schlucke_for_this_beer) = hits_and_schlucke_for_this_beer(&strafbeer_hit, second.hits, &strafbeer_schluck, second.schluck_drank, &add.source.id);
        data.add_finished_beer(second.clean, hits_for_this_beer, schlucke_for_this_beer, additional_hit_from_strafbeer);
    }
}

fn hits_and_schlucke_for_this_beer(strafbeer_hit: &HashMap<u32, u32>, hit_amount: u32, strafbeer_schlucke: &HashMap<u32, u32>, schlucke_amount: u32, key: &u32) -> (u32, u32) {
    (hits_used_for_this_beer(strafbeer_hit, hit_amount, key), schlucke_used_for_this_beer(strafbeer_schlucke, schlucke_amount, key))
}

fn hits_used_for_this_beer(strafbeer_hit: &HashMap<u32, u32>, hit_amount: u32, key: &u32) -> u32 {
    if strafbeer_hit.contains_key(key) {
        hit_amount - strafbeer_hit.get(key).unwrap()
    } else {
        hit_amount
    }
}

fn schlucke_used_for_this_beer(strafbeer_schlucke: &HashMap<u32, u32>, schlucke_amount: u32, key: &u32) -> u32 {
    if strafbeer_schlucke.contains_key(key) {
        schlucke_amount - strafbeer_schlucke.get(key).unwrap()
    } else {
        schlucke_amount
    }
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;
    use super::*;
    use crate::team_player_data::*;
    use crate::util::test::{game_1st_finish_2straf, game_2nd_finish, game_3rd_finish};


    #[test]
    fn test_strafschluck_0_point_five() {
        let games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2), game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2)];
        let teams = vec![TEST_TEAM1, TEST_TEAM2];
        let data = calculate_strafschluck(&games, &teams);
        data.print();
        let strafschluck_effect = data.effect_of_single_schluck();
        assert!(approx_eq!(f32, strafschluck_effect, 0.5, ulps=2));
    }
    #[test]
    fn test_strafschluck_0_point_75() {
        let games = vec![game_3rd_finish(TEST_TEAM1, TEST_TEAM2), game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2), game_2nd_finish(TEST_TEAM1, TEST_TEAM2)];
        let teams = vec![TEST_TEAM1, TEST_TEAM2];
        let data = calculate_strafschluck(&games, &teams);
        data.print();
        let strafschluck_effect = data.effect_of_single_schluck();
        assert!(approx_eq!(f32, strafschluck_effect, 0.75, ulps=2));
    }
}
