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
    //was for testing regarding first throw let games24_no_dos_no_wedel = create_all_games_without_dos_or_wedelmedel();
    //was for testing regarding first throw let games_24_no_sebi_or_me = create_games_without_me_or_sebi();
    //print_throwing_accuracy(&games_24, &all_teams, &all_players);
    //print_side_information(&games_24);
    //print_first_throw_effect(&games24);
    //print_team_first_throws(&games24, &all_teams);
    print_complete_drinking_speed(&games24, &all_players, &all_teams, 0.5);
}



