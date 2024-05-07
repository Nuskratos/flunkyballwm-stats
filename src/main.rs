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


