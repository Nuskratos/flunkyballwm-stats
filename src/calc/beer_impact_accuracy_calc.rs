use crate::calc::beer_impact_accuracy_data::{EntityBeerImpact, TournamentEntityBeerImpact};
use crate::data::{AdditionalType, Game, NamedEntity};
use crate::util::team_from_player;
use std::collections::HashMap;

pub fn calculate_beer_impact_accuracy_for_one_tournament(
    games: &Vec<Game>,
) -> TournamentEntityBeerImpact {
    let mut ret_val = TournamentEntityBeerImpact {
        impacts: HashMap::new(),
    };
    let mut player_beer_map: HashMap<NamedEntity, u32> = HashMap::new();
    for game in games {
        // Add accuracy for Player beer count
        for round in &game.rounds {
            let player_beers = player_beer_map
                .get(&round.thrower.named_entity)
                .unwrap_or(&0);
            let player_entry = ret_val
                .impacts
                .entry(round.thrower.named_entity.clone())
                .or_insert(EntityBeerImpact::new(round.thrower.named_entity.clone()));
            player_entry.add_throw(round.hit, *player_beers as usize);
            let team = team_from_player(round.thrower.named_entity.id, &game)
                .named_entity
                .to_owned();
            let team_entry = ret_val
                .impacts
                .entry(team)
                .or_insert(EntityBeerImpact::new(team));
            team_entry.add_throw(round.hit, *player_beers as usize);
        }
        //Add beers to player stats (normal + straf)
        *player_beer_map.entry(game.left_1.named_entity).or_insert(0) += 1;
        *player_beer_map.entry(game.left_2.named_entity).or_insert(0) += 1;
        *player_beer_map
            .entry(game.right_1.named_entity)
            .or_insert(0) += 1;
        *player_beer_map
            .entry(game.right_2.named_entity)
            .or_insert(0) += 1;
        for additional in &game.additionals_vec() {
            match additional.additional.kind {
                AdditionalType::STRAFBIER => {
                    *player_beer_map
                        .entry(additional.additional.source.named_entity)
                        .or_insert(0) += 1
                }
                _ => {}
            }
        }
    }
    ret_val
}

pub fn calculate_beer_impact_accuracy(tournaments: &Vec<Vec<Game>>) -> TournamentEntityBeerImpact {
    let mut ret_val = TournamentEntityBeerImpact {
        impacts: HashMap::new(),
    };
    for tournament in tournaments {
        ret_val.merge(&calculate_beer_impact_accuracy_for_one_tournament(
            tournament,
        ));
    }
    ret_val
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::NamedEntity;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{
        game_2nd_finish, game_2nd_finish_enemy_miss, game_5th_finish_strafbeer,
        game_finished_after_everyone_missed_first,
    };

    #[test]
    fn test_simple_beer_impact_accuracy_calculation() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2),
        ];
        let data = calculate_beer_impact_accuracy(&vec![games]);
        let t1_1 = data.impacts.get(&TEST_TEAM1.member_1.named_entity).unwrap();
        let t1_2 = data.impacts.get(&TEST_TEAM1.member_2.named_entity).unwrap();
        let t2_1 = data.impacts.get(&TEST_TEAM2.member_1.named_entity).unwrap();
        //        let t2_2 = data.impacts.get(&TEST_TEAM2.member_2.named_entity).unwrap();
        let team1 = data.impacts.get(&TEST_TEAM1.named_entity).unwrap();
        let team2 = data.impacts.get(&TEST_TEAM2.named_entity).unwrap();

        assert_eq!(t1_1.accuracy_points[0].hits, 1);
        assert_eq!(t1_1.accuracy_points[0].throws, 1);
        assert_eq!(t1_2.accuracy_points[0].hits, 1);
        assert_eq!(t1_2.accuracy_points[0].throws, 1);
        assert_eq!(t2_1.accuracy_points[0].hits, 1);
        assert_eq!(t2_1.accuracy_points[0].throws, 1); // All hit in first game
                                                       //        assert_eq!(t2_2.accuracy_points[0].hits, 0);
                                                       //       assert_eq!(t2_2.accuracy_points[0].throws, 0);

        assert_eq!(t1_1.accuracy_points[1].hits, 1);
        assert_eq!(t1_1.accuracy_points[1].throws, 1);
        assert_eq!(t1_2.accuracy_points[1].hits, 1);
        assert_eq!(t1_2.accuracy_points[1].throws, 1);
        assert_eq!(t2_1.accuracy_points[1].hits, 0); // missed his throw
        assert_eq!(t2_1.accuracy_points[1].throws, 1);
        //        assert_eq!(t2_2.accuracy_points[1].hits, 0);
        //        assert_eq!(t2_2.accuracy_points[1].throws, 0);

        let raw_beer_impact = data.raw_points();

        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].hits, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].throws, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].hits, 2);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].throws, 3);
    }

    #[test]
    fn test_larger_beer_impact_accuracy_calculation() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2),
            game_finished_after_everyone_missed_first(TEST_TEAM1, TEST_TEAM2),
        ];
        let data = calculate_beer_impact_accuracy(&vec![games]);
        let t1_1 = data.impacts.get(&TEST_TEAM1.member_1.named_entity).unwrap();
        let t1_2 = data.impacts.get(&TEST_TEAM1.member_2.named_entity).unwrap();
        let t2_1 = data.impacts.get(&TEST_TEAM2.member_1.named_entity).unwrap();
        let t2_2 = data.impacts.get(&TEST_TEAM2.member_2.named_entity).unwrap();
        let team1 = data.impacts.get(&TEST_TEAM1.named_entity).unwrap();
        let team2 = data.impacts.get(&TEST_TEAM2.named_entity).unwrap();

        assert_eq!(t1_1.accuracy_points[0].hits, 1);
        assert_eq!(t1_1.accuracy_points[0].throws, 1);
        assert_eq!(t1_2.accuracy_points[0].hits, 1);
        assert_eq!(t1_2.accuracy_points[0].throws, 1);
        assert_eq!(t2_1.accuracy_points[0].hits, 1);
        assert_eq!(t2_1.accuracy_points[0].throws, 1); // All hit in first game

        assert_eq!(t1_1.accuracy_points[1].hits, 1);
        assert_eq!(t1_1.accuracy_points[1].throws, 1);
        assert_eq!(t1_2.accuracy_points[1].hits, 1);
        assert_eq!(t1_2.accuracy_points[1].throws, 1);
        assert_eq!(t2_1.accuracy_points[1].hits, 0); // missed their throw
        assert_eq!(t2_1.accuracy_points[1].throws, 1);

        assert_eq!(t1_1.accuracy_points[2].hits, 1);
        assert_eq!(t1_1.accuracy_points[2].throws, 2);
        assert_eq!(t1_2.accuracy_points[2].hits, 1);
        assert_eq!(t1_2.accuracy_points[2].throws, 2);
        assert_eq!(t2_1.accuracy_points[2].hits, 1);
        assert_eq!(t2_1.accuracy_points[2].throws, 2);
        assert_eq!(t2_2.accuracy_points[2].hits, 0);
        assert_eq!(t2_2.accuracy_points[2].throws, 1);

        let raw_beer_impact = data.raw_points();

        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].hits, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].throws, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].hits, 2);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].throws, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[2].hits, 3);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[2].throws, 7);
    }

    #[test]
    fn test_with_strafbeer() {
        let games = vec![
            game_5th_finish_strafbeer(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_finished_after_everyone_missed_first(TEST_TEAM2, TEST_TEAM3),
        ];
        let data = calculate_beer_impact_accuracy(&vec![games]);
        let t1_1 = data.impacts.get(&TEST_TEAM1.member_1.named_entity).unwrap();
        let t1_2 = data.impacts.get(&TEST_TEAM1.member_2.named_entity).unwrap();
        let t2_1 = data.impacts.get(&TEST_TEAM2.member_1.named_entity).unwrap();
        let t2_2 = data.impacts.get(&TEST_TEAM2.member_2.named_entity).unwrap();
        let t3_1 = data.impacts.get(&TEST_TEAM3.member_1.named_entity).unwrap();
        let t3_2 = data.impacts.get(&TEST_TEAM3.member_2.named_entity).unwrap();
        let team1 = data.impacts.get(&TEST_TEAM1.named_entity).unwrap();
        let team2 = data.impacts.get(&TEST_TEAM2.named_entity).unwrap();
        let team3 = data.impacts.get(&TEST_TEAM3.named_entity).unwrap();

        assert_eq!(t1_1.accuracy_points[0].hits, 2);
        assert_eq!(t1_1.accuracy_points[0].throws, 2);
        assert_eq!(t1_2.accuracy_points[0].hits, 1);
        assert_eq!(t1_2.accuracy_points[0].throws, 1);
        assert_eq!(t3_1.accuracy_points[0].hits, 1);
        assert_eq!(t3_1.accuracy_points[0].throws, 1);
        assert_eq!(t3_2.accuracy_points[0].hits, 1);
        assert_eq!(t3_2.accuracy_points[0].throws, 1);

        assert_eq!(t1_1.accuracy_points[2].hits, 1);
        assert_eq!(t1_1.accuracy_points[2].throws, 1);
        assert_eq!(t1_2.accuracy_points[1].hits, 1);
        assert_eq!(t1_2.accuracy_points[1].throws, 1);
        assert_eq!(t2_1.accuracy_points[0].hits, 1);
        assert_eq!(t2_1.accuracy_points[0].throws, 1);

        assert_eq!(t2_1.accuracy_points[1].hits, 1);
        assert_eq!(t2_1.accuracy_points[1].throws, 2);
        assert_eq!(t2_2.accuracy_points[1].hits, 1);
        assert_eq!(t2_2.accuracy_points[1].throws, 2);
        assert_eq!(t3_1.accuracy_points[1].hits, 1);
        assert_eq!(t3_1.accuracy_points[1].throws, 2);
        assert_eq!(t3_2.accuracy_points[1].hits, 0);
        assert_eq!(t3_2.accuracy_points[1].throws, 1);

        let raw_beer_impact = data.raw_points();

        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].hits, 6);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[0].throws, 6);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].hits, 4);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[1].throws, 8);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[2].hits, 1);
        assert_eq!(raw_beer_impact.accuracy_at_beers_drank[2].throws, 1);
    }
}
