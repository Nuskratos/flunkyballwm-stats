use crate::calc::side_information_data::{SideInformation, SideSplit};
use crate::data::AdditionalType::{STRAFBIER, STRAFSCHLUCK};
use crate::data::Game;

pub fn calc_side_information(games: &Vec<Game>) -> SideSplit {
    let mut left: SideInformation = Default::default();
    let mut right: SideInformation = Default::default();
    for game in games {
        if game.result.points_left > game.result.points_right {
            left.wins += 1;
        } else {
            right.wins += 1;
        }
        left.points += game.result.points_left;
        right.points += game.result.points_right;
        for round in &game.rounds {
            if round.thrower.id == game.left_1.id || round.thrower.id == game.left_2.id {
                left.throws += 1;
                if round.hit {
                    left.hits += 1;
                }
            } else {
                right.throws += 1;
                if round.hit {
                    right.hits += 1;
                }
            }
            for additional in &round.additionals {
                match &additional.kind {
                    STRAFSCHLUCK => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            left.schluck += 1;
                        } else {
                            right.schluck += 1;
                        }
                    }
                    STRAFBIER => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            left.beer += 1;
                        } else {
                            right.beer += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    SideSplit{left, right}
}

#[cfg(test)]
mod test{
    use crate::calc::side_information_calc::calc_side_information;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_1st_finish_2straf, game_2nd_finish, game_2nd_finish_enemy_miss, game_3rd_finish};

    #[test]
    fn basic(){
        let games = vec![game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2), game_3rd_finish(TEST_TEAM2, TEST_TEAM3), game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2)];
        let data = calc_side_information(&games);
        assert_eq!(data.left.schluck, 0);
        assert_eq!(data.left.beer, 0);
        assert_eq!(data.left.throws, 6);
        assert_eq!(data.left.hits, 6);
        assert_eq!(data.left.wins, 3);
        assert_eq!(data.left.points, 42);

        assert_eq!(data.right.schluck, 2);
        assert_eq!(data.right.beer, 0);
        assert_eq!(data.right.throws, 4);
        assert_eq!(data.right.hits, 3);
        assert_eq!(data.right.wins, 0);
        assert_eq!(data.right.points, 0);
    }
}