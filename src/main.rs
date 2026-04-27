use std::fmt::Debug;
use chrono::Local;
use serde::Serialize;
use data::*;
use wm24::*;
use crate::calc::accuracy_after_running_calc::calculate_accuracy_after_running;
use crate::calc::accuracy_calc::calc_enemy_accuracy;
use crate::calc::beer_impact_accuracy_calc::calculate_beer_impact_accuracy;
use crate::calc::calculation::{calculate_throwing_accuracy};
use crate::calc::chain_calc::calculate_hit_and_miss_chains_team_player;
use crate::calc::drink_calc::calculate_drinking_speed;
use crate::calc::penalties_calc::calculate_amount_of_penalties;
use crate::calc::ppg_calc::calculate_amount_of_points_per_game;
use crate::calc::running_calc::calculate_running_speeds;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::first_throw_calc::{calc_general_first_throw, calc_team_first_throws};
use crate::calc::rock_paper_scissors_calc::calculate_rock_paper_scissors;
use crate::calc::side_information_calc::calc_side_information;
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

fn print_all_calcs(games : &Vec<Vec<Game>>){
    let flattened = games.into_iter().flatten().cloned().collect();
    let all_players = players_from_games(&flattened);
    let all_teams = teams_from_games(&flattened);
    calculate_throwing_accuracy(&flattened).print();
    calc_side_information(&flattened).print();
    calc_general_first_throw(&flattened).print();
    calc_team_first_throws(&flattened).print();
    let strafschluck_data = calculate_strafschluck(&flattened);
    strafschluck_data.print();
    calculate_drinking_speed(&flattened, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).print();
    calculate_throws_per_game(&flattened).print();
    calc_enemy_accuracy(&flattened).print();
    calculate_hit_and_miss_chains_team_player(&flattened).print();
    //print_amount_of_penalties(&games, &all_teams, &all_players);
    let penalties_stats = calculate_amount_of_penalties(&flattened);
    penalties_stats.print();
    let ppg_stats = calculate_amount_of_points_per_game(&flattened);
    ppg_stats.print();
    let running_speeds = calculate_running_speeds(&flattened, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck());
    calculate_rock_paper_scissors(&flattened).print();
    running_speeds.print();
    calculate_beer_impact_accuracy(&games).print();
    calculate_accuracy_after_running(&flattened).print();
}
fn create_csv_for_calcs(games : &Vec<Vec<Game>>, fileprefix : String, date : &String){
    let flattened = games.into_iter().flatten().cloned().collect();
    let all_players = players_from_games(&flattened);
    let all_teams = teams_from_games(&flattened);
    let strafschluck_data = calculate_strafschluck(&flattened);

    strafschluck_data.serialize(&fileprefix, &date);
    calculate_throwing_accuracy(&flattened).serialize(&fileprefix,&date);
    calc_side_information(&flattened).serialize(&fileprefix, &date);
    calc_general_first_throw(&flattened).serialize(&fileprefix,&date);
    calc_team_first_throws(&flattened).serialize(&fileprefix,&date);
    calculate_drinking_speed(&flattened, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).serialize(&fileprefix,&date);
    calculate_throws_per_game(&flattened).serialize(&fileprefix,&date);
    calc_enemy_accuracy(&flattened).serialize(&fileprefix,&date);
    calculate_hit_and_miss_chains_team_player(&flattened).serialize(&fileprefix, &date);
    calculate_amount_of_penalties(&flattened).serialize(&fileprefix, &date);
    calculate_amount_of_points_per_game(&flattened).serialize(&fileprefix, &date);
    calculate_running_speeds(&flattened, &all_players, &all_teams, strafschluck_data.effect_of_single_schluck()).serialize(&fileprefix, &date);
    calculate_rock_paper_scissors(&flattened).serialize(&fileprefix, &date);
    calculate_beer_impact_accuracy(&games).serialize(&fileprefix, &date);
    calculate_accuracy_after_running(&flattened).serialize(&fileprefix, &date);
}


fn print_wm25(){
    let games = vec![create_all_games_wm_2025()];
    print_all_calcs(&games);
}

fn print_wm24(){
    let games = vec![create_wm24_no_illegal()];
    print_all_calcs(&games);
}
fn print_hamburg_24(){
    let games = vec![create_spassturnier_24()];
    print_all_calcs(&games);
}
fn wm_stats()->Vec<Vec<Game>>{
    vec![create_wm24_no_illegal(), create_all_games_wm_2025()]
}
fn non_wm_stats()->Vec<Vec<Game>>{
    vec![create_spassturnier_24()]
}
fn all_games() ->Vec<Vec<Game>>{
    let mut all_games = wm_stats();
    all_games.extend(non_wm_stats());
    all_games
}

fn print_total_stats(){
    print_all_calcs(&all_games());
}

fn print_wm_stats(){
    print_all_calcs(&wm_stats());
}
fn create_csv_of_statistics(){
    let date = Local::now().format("%Y-%m-%d").to_string();
    // All games
    let all_games = all_games();
    create_csv_for_calcs(&all_games, "all_games".to_string(), &date);
    // AlL WM
    let all_wm = wm_stats();
    create_csv_for_calcs(&all_wm, "all_wm".to_string(), &date);
    // All non VM
    let all_non_wm = non_wm_stats();
    create_csv_for_calcs(&all_non_wm, "all_non_wm".to_string(), &date);

    // Every single tournament
    let wm_2024 = vec![create_wm24_no_illegal()];
    create_csv_for_calcs(&wm_2024, "wm24".to_string(), &date);
    let wm_25 = vec![create_all_games_wm_2025()];
    create_csv_for_calcs(&wm_25, "wm25".to_string(), &date);
    let spass24 = vec![create_spassturnier_24()];
    create_csv_for_calcs(&spass24, "spass24".to_string(), &date);

}
fn main() {
    print_total_stats();
    create_csv_of_statistics()
}



