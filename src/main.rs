mod data;
mod wm24;
mod team_player_data;
mod test_stuff;
mod calc;
mod util;
mod fake_game;

use std::fmt::Debug;
use data::*;
use wm24::*;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::calculation::{print_amount_of_penalties, print_amount_of_points_per_game, print_average_throws_per_game, print_enemy_accuracy, print_first_throw_effect, print_hit_and_miss_chains, print_side_information, print_team_first_throws, print_throwing_accuracy};
use crate::calc::drink_calc::calculate_drinking_speed;
use crate::calc::running_calc::calculate_running_speeds;
use crate::team_player_data::{BEEF, CHRIS, DA_HAM_SIE, DOS_BROS, FLO, GEWERTET, HANNES, JEROME, JONAS, LAURA, LUISE, MALTE, SASCHA, SEBI, STRAMMSEIN, TOBIAS, WEDELMEDEL, WHITE_CLAW};
use crate::test_stuff::test_first_throw_value;
use crate::util::{players_from_games, teams_from_games};

fn main() {
    let games24 = create_no_gewertet();
    let all_players = players_from_games(&games24);
    let all_teams = teams_from_games(&games24);
    // Checking single game games24.iter().find(|x| x.match_number == 21).unwrap().print();
    //was for testing regarding first throw let games24_no_dos_no_wedel = create_all_games_without_dos_or_wedelmedel();
    //was for testing regarding first throw let games_24_no_sebi_or_me = create_games_without_me_or_sebi();
    print_throwing_accuracy(&games24, &all_teams, &all_players);
    print_side_information(&games24);
    print_first_throw_effect(&games24);
    print_team_first_throws(&games24, &all_teams);
    let strafschluck_data = calculate_strafschluck(&games24, &all_teams);
    let schluck_effect = strafschluck_data.effect_of_single_schluck();
    strafschluck_data.print();
    calculate_drinking_speed(&games24, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).print();
    print_average_throws_per_game(&games24, &all_teams, &all_players);
    print_enemy_accuracy(&games24, &all_teams);
    print_hit_and_miss_chains(&games24, &all_teams, &all_players);
    print_amount_of_penalties(&games24, &all_teams, &all_players);
    print_amount_of_points_per_game(&games24, &all_teams, &all_players);
    let running_speeds = calculate_running_speeds(&games24, &all_players, &all_teams, schluck_effect);
    running_speeds.print();
}



