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