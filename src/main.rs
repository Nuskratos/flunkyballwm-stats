use std::fmt::Debug;
use chrono::Local;
use serde::Serialize;
use data::*;
use wm24::*;
use crate::calc::accuracy_calc::calc_enemy_accuracy;
use crate::calc::calculation::{csv_side_information, csv_throwing_accuracy, print_first_throw_effect, print_side_information, print_throwing_accuracy};
use crate::calc::chain_calc::calculate_hit_and_miss_chains_team_player;
use crate::calc::drink_calc::calculate_drinking_speed;
use crate::calc::penalties_calc::calculate_amount_of_penalties;
use crate::calc::ppg_calc::calculate_amount_of_points_per_game;
use crate::calc::running_calc::calculate_running_speeds;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::first_throw_calc::calc_team_first_throws;
use crate::calc::throw_per_game_calc::calculate_throws_per_game;
use crate::hamburg24::create_spassturnier_24;
use crate::util::{players_from_games, teams_from_games};
use crate::wm25::create_all_games_wm_2025;

mod data;
mod wm24;
pub mod team_player_data;
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
    calc_team_first_throws(&games).print();
    let strafschluck_data = calculate_strafschluck(&games, &all_teams);
    strafschluck_data.print();
    calculate_drinking_speed(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).print();
    calculate_throws_per_game(&games).print();
    calc_enemy_accuracy(&games).print();
    calculate_hit_and_miss_chains_team_player(&games).print();
    //print_amount_of_penalties(&games, &all_teams, &all_players);
    let penalties_stats = calculate_amount_of_penalties(&games);
    penalties_stats.print();
    let ppg_stats = calculate_amount_of_points_per_game(&games);
    ppg_stats.print();
    // TODO print_rock_paper_scissorvalues(&games, &all_teams, &all_players); // Generelles und beste/schlechteste matchups
    let running_speeds = calculate_running_speeds(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck());
    running_speeds.print();
}
fn create_csv_for_calcs(games : &Vec<Game>, fileprefix : String, date : String){
    let all_players = players_from_games(&games);
    let all_teams = teams_from_games(&games);
    csv_throwing_accuracy(&games, &all_teams, &all_players, &fileprefix, &date);
    csv_side_information(&games, &fileprefix, &date);
    print_first_throw_effect(&games);// TODO
    calc_team_first_throws(&games).serialize(&fileprefix,&date);
    let strafschluck_data = calculate_strafschluck(&games, &all_teams);
    strafschluck_data.serialize(&fileprefix, &date);
    calculate_drinking_speed(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).serialize(&fileprefix,&date);
    calculate_throws_per_game(&games).serialize(&fileprefix,&date);
    calc_enemy_accuracy(&games).serialize(&fileprefix,&date);
    calculate_hit_and_miss_chains_team_player(&games).serialize(&fileprefix, &date);
    calculate_amount_of_penalties(&games).serialize(&fileprefix, &date);
    calculate_amount_of_points_per_game(&games).serialize(&fileprefix, &date);
    // TODO print_rock_paper_scissorvalues(&games, &all_teams, &all_players); // Generelles und beste/schlechteste matchups
    calculate_running_speeds(&games, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).serialize(&fileprefix, &date);
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
fn total_stats()->Vec<Game>{
    let mut games = create_no_gewertet();
    games.append(&mut create_spassturnier_24());
    games.append(&mut create_all_games_wm_2025());
    games
}
fn wm_stats()->Vec<Game>{
    let mut games = create_no_gewertet();
    games.append(&mut create_all_games_wm_2025());
    games
}

fn print_total_stats(){
    print_all_calcs(&total_stats());
}

fn print_wm_stats(){
    print_all_calcs(&wm_stats());
}
fn create_csv_of_statistics(){
    let date = Local::now().format("%Y-%m-%d").to_string();
    // All games
    let all_games = total_stats();
    create_csv_for_calcs(&all_games, "all_games".to_string(), date);
    // AlL WM

    // All non VM

    // Every single tournament


}
fn main() {
    print_total_stats();
    create_csv_of_statistics()
}



