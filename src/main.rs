pub mod data;
mod wm24;
mod team_player_data;
mod calculation;
mod test_stuff;

use std::fmt::Debug;
use data::*;
use wm24::*;
use crate::calculation::{print_complete_drinking_speed, print_first_throw_effect, print_side_information, print_team_first_throws, print_throwing_accuracy};
use crate::team_player_data::{BEEF, CHRIS, DA_HAM_SIE, DOS_BROS, FLO, GEWERTET, HANNES, JEROME, JONAS, LAURA, LUISE, MALTE, SASCHA, SEBI, STRAMMSEIN, TOBIAS, WEDELMEDEL, WHITE_CLAW};
use crate::test_stuff::test_first_throw_value;

fn main() {
    let all_players: Vec<TeamMember> = vec![JEROME, BEEF, SEBI, FLO, SASCHA, JONAS, LUISE, TOBIAS, MALTE, CHRIS, HANNES, LAURA];
    let all_teams: Vec<Team> = vec![DA_HAM_SIE, DOS_BROS, STRAMMSEIN, WHITE_CLAW, WEDELMEDEL, GEWERTET];
    let games24 = create_all_games_wm_2024();
    // Checking single games games24.iter().find(|x| x.match_number == 21).unwrap().print();
    //let games24_no_dos_no_wedel = create_all_games_without_dos_or_wedelmedel();
    //let games_24_no_sebi_or_me = create_games_without_me_or_sebi();
    //print_throwing_accuracy(&games_24, &all_teams, &all_players);
    //print_side_information(&games_24);
    //print_first_throw_effect(&games24);
    //print_team_first_throws(&games24, &all_teams);
    print_complete_drinking_speed(&games24, &all_players, &all_teams, 0.5);
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


