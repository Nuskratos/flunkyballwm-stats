use std::fmt::Debug;

use data::*;
use wm24::*;

use crate::calc::calculation::{print_amount_of_penalties, print_amount_of_points_per_game, print_average_throws_per_game, print_enemy_accuracy, print_first_throw_effect, print_hit_and_miss_chains, print_side_information, print_team_first_throws, print_throwing_accuracy};
use crate::calc::drink_calc::calculate_drinking_speed;
use crate::calc::running_calc::calculate_running_speeds;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::hamburg24::create_spassturnier_24;
use crate::util::{players_from_games, teams_from_games};
use crate::wm25::create_all_games_wm_2025;

mod data;
mod wm24;
mod team_player_data;
mod test_stuff;
mod calc;
mod util;
mod fake_game;
mod hamburg24;
mod wm25;

fn print_all_calcs(games : &Vec<Game>){
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
    print_enemy_accuracy(&games);
    print_hit_and_miss_chains(&games, &all_teams, &all_players);
    print_amount_of_penalties(&games, &all_teams, &all_players);
    print_amount_of_points_per_game(&games, &all_teams, &all_players);
    // TODO print_rock_paper_scissorvalues(&games, &all_teams, &all_players); // Generelles und beste/schlechteste matchups
    let running_speeds = calculate_running_speeds(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck());
    running_speeds.print();
}


fn print_wm25(){
    let games = create_all_games_wm_2025();
    print_all_calcs(&games);
}

fn print_wm24(){
    let games = create_no_gewertet();
    print_all_calcs(&games);
}
fn print_hamburg_24(){
    let games = create_spassturnier_24();
    print_all_calcs(&games);
}

fn print_total_stats(){
    let mut games = create_no_gewertet();
    games.append(&mut create_spassturnier_24());
    games.append(&mut create_all_games_wm_2025());
    print_all_calcs(&games);
}

fn print_wm_stats(){
    let mut games = create_no_gewertet();
    games.append(&mut create_all_games_wm_2025());
    print_all_calcs(&games);
}
fn main() {
    print_wm_stats()
}



