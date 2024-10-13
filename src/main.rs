mod data;
mod wm24;
mod team_player_data;
mod test_stuff;
mod calc;
mod util;
mod fake_game;
mod hamburg24;

use std::fmt::Debug;
use data::*;
use wm24::*;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::calculation::{print_amount_of_penalties, print_amount_of_points_per_game, print_average_throws_per_game, print_enemy_accuracy, print_first_throw_effect, print_hit_and_miss_chains, print_side_information, print_team_first_throws, print_throwing_accuracy};
use crate::calc::drink_calc::calculate_drinking_speed;
use crate::calc::running_calc::calculate_running_speeds;
use crate::hamburg24::create_spassturnier_24;
use crate::team_player_data::{BEEF, CHRIS, DA_HAM_SIE, DOS_BROS, FLO, GEWERTET, HANNES, JEROME, JONAS, LAURA, LUISE, MALTE, SASCHA, SEBI, STRAMMSEIN, TOBIAS, WEDELMEDEL, WHITE_CLAW};
use crate::test_stuff::test_first_throw_value;
use crate::util::{players_from_games, teams_from_games};

fn print_all_wm24(){
    let games = create_no_gewertet();
    let all_players = players_from_games(&games);
    let all_teams = teams_from_games(&games);
    // Checking single game games.iter().find(|x| x.match_number == 21).unwrap().print();
    //was for testing regarding first throw let games_no_dos_no_wedel = create_all_games_without_dos_or_wedelmedel();
    //was for testing regarding first throw let games_24_no_sebi_or_me = create_games_without_me_or_sebi();
    print_throwing_accuracy(&games, &all_teams, &all_players);
    print_side_information(&games);
    print_first_throw_effect(&games);
    print_team_first_throws(&games, &all_teams);
    let strafschluck_data = calculate_strafschluck(&games, &all_teams);
    let schluck_effect = strafschluck_data.effect_of_single_schluck();
    strafschluck_data.print();
    calculate_drinking_speed(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).print();
    print_average_throws_per_game(&games, &all_teams, &all_players);
    print_enemy_accuracy(&games, &all_teams);
    print_hit_and_miss_chains(&games, &all_teams, &all_players);
    print_amount_of_penalties(&games, &all_teams, &all_players);
    print_amount_of_points_per_game(&games, &all_teams, &all_players);
    let running_speeds = calculate_running_speeds(&games, &all_players, &all_teams, schluck_effect);
    running_speeds.print();
}

fn main() {
    let games = create_spassturnier_24();
    let all_players = players_from_games(&games);
    let all_teams = teams_from_games(&games);
    print_throwing_accuracy(&games, &all_teams, &all_players);
    print_side_information(&games);
    print_first_throw_effect(&games);
    print_team_first_throws(&games, &all_teams);
    let strafschluck_data = calculate_strafschluck(&games, &all_teams);
    strafschluck_data.print();
    calculate_drinking_speed(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).print();
    print_average_throws_per_game(&games, &all_teams, &all_players);
    print_enemy_accuracy(&games, &all_teams);
    print_hit_and_miss_chains(&games, &all_teams, &all_players);
    print_amount_of_penalties(&games, &all_teams, &all_players);
    print_amount_of_points_per_game(&games, &all_teams, &all_players);
    // This one seems to be a bit faulty calculated
    let running_speeds = calculate_running_speeds(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck());
    running_speeds.print();
}



