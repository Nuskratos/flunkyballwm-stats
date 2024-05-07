pub mod data;
mod wm24;
mod team_player_data;

use std::arch::x86_64::_bittestandcomplement;
use std::fmt::Debug;
use AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use data::*;
use wm24::*;

fn main() {
    let games_24 = create_all_games_wm_2024();
    games_24.last().expect("expected last element to be there").print();


}

/*
fn game_(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = ;
    let additionals = vec![];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(), additionals, left_began);
    Game {result, match_number : , left_team, left_1, left_2, right_team, right_1, right_2, rounds}
}
*/


fn first_game_new(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals: Vec<ARC> = vec![ARC::schluck(left_1.clone(), 0), ARC::finish(left_1.clone(), 14), ARC::finish(left_2.clone(), 14)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true], additionals.clone(), left_began);
    Game { result, match_number: 1, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

fn second_game_new(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let mut additionals = vec![ARC::finish(right_1.clone(), 4), ARC::finish(right_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, true, true, true, false, true], additionals.clone(), left_began);
    Game { result, match_number: 2, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

fn game_3(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 6), ARC::finish(left_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, true, true, false, false, true, true], additionals, left_began);
    Game { result, match_number: 3, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

fn game_4(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 6), ARC::finish(right_1.clone(), 7), ARC::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![false, false, true, true, true, false, true, true, true], additionals, left_began);
    Game { result, match_number: 4, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}



fn first_game_old(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let mut rounds = create_normalized_rounds(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true]);
    let straf_sasch = Additional { kind: STRAFSCHLUCK, source: left_1.clone() };
    rounds.first_mut().unwrap().additionals(vec![straf_sasch]);

    let finished_sasch = Additional { kind: FINISHED, source: left_1.clone() };
    let finished_jonas = Additional { kind: FINISHED, source: left_2.clone() };
    rounds.last_mut().unwrap().additionals(vec![finished_sasch, finished_jonas]);
    let result = Result { points_left: 14, points_right: 0 };
    Game { result, match_number: 1, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

//THIS IS FAULTY, because Florian was finished but according to this sheet ran in the 2nd to last throw
fn second_game_old_faulty(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let result = Result { points_left: 0, points_right: 14 };
    let mut rounds = create_normalized_rounds(right_1.clone(), right_2.clone(), left_1.clone(), left_2.clone(), vec![true, true, true, true, true, false, true]); //
    let finish_flo = Additional { kind: FINISHED, source: right_1.clone() };
    let finish_sebi = Additional::finish(right_2.clone());
    rounds.get_mut(4).unwrap().additionals(vec![finish_flo]);
    rounds.last_mut().unwrap().additionals(vec![finish_sebi]);
    Game { result, left_1, left_2, right_1, right_2, match_number: 2, left_team, right_team, rounds }
}