use std::collections::HashMap;
use std::{error::Error, io, process};
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::Writer;
use crate::calc::accuracy_data::{Accuracy};
use crate::calc::chain_calc::calculate_hit_and_miss_chains_team_player;
use crate::calc::drink_total_data::PlayerDrinkingSpeed;
use crate::calc::first_throw_data::FirstThrows;
use crate::calc::penalties_calc::calculate_amount_of_penalties;
use crate::calc::ppg_calc::calculate_amount_of_points_per_game;
use crate::calc::side_information_calc::calc_side_information;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::throw_per_game_calc::calculate_throws_per_game;
use crate::data::{Game, NamedEntity, Team, TeamMember};
use crate::util::{print_line_break, team_from_player, team_id_from_player, team_name_from_id};
use crate::team_player_data::NAME_WIDTH; // Used in printlines

pub fn percentage(divisor: usize, divident: usize) -> f32 { divisor as f32 / divident as f32 * 100.0 }

pub fn average(divisor: u32, dividend: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average(dividend: u32, divisor: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn calculate_throwing_accuracy(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) -> Vec<Accuracy>{
    let mut throws = 0;
    let mut hits = 0;
    let mut player_scores :HashMap<NamedEntity, Accuracy> = HashMap::new();
    let mut team_scores :HashMap<NamedEntity, Accuracy> = HashMap::new();

    for game in games {
        for round in &game.rounds {
            let team_entity_from_player = team_from_player(round.thrower.id(), game).named_entity.to_owned();
            let player_accuracy = player_scores.entry(round.thrower.named_entity.to_owned()).or_insert(Accuracy{named_entity: round.thrower.named_entity.to_owned(), hits:0, throws:0});
            let team_accuracy = team_scores.entry(team_entity_from_player.to_owned()).or_insert(Accuracy{named_entity: team_entity_from_player, hits:0, throws:0});
            throws = throws + 1;
            player_accuracy.throws += 1;
            team_accuracy.throws +=1;
            if round.hit {
                hits = hits + 1;
                player_accuracy.hits += 1;
                team_accuracy.hits +=1;
            }
        }
    }

    let mut result_vec: Vec<Accuracy> = Vec::new();
    for score in team_scores {
        result_vec.push(score.1);
    }
    for score in player_scores {
        result_vec.push(score.1);
    }
    result_vec.push(Accuracy{throws:throws, hits:hits, named_entity: NamedEntity{name:"Average", alias:"Average", id:999 }});
    result_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_vec.reverse();
    return result_vec;
}

pub fn print_throwing_accuracy(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let result_vec = calculate_throwing_accuracy(games, teams, players);
    println!("Throwing accuracy:");
    print_line_break(70);
    for accuracy in &result_vec {
        accuracy.print();
    }
    print_line_break(70);
    println!();
}

pub fn open_writer(filename : String) -> (Writer<File>, bool){
    let path = Path::new(&filename);
    let file_exists = path.exists();
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Couldn't open file");
    let mut wtr = csv::Writer::from_writer(file);
    (wtr, file_exists)
}

pub fn csv_throwing_accuracy(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>, fileprefix : &String, date : &String){
    let result_vec = calculate_throwing_accuracy(games, teams, players);
    let (mut wtr, file_exists) = open_writer(date.to_string()+"throwing_accuracy.csv");
    if !file_exists{
        wtr.write_record(&["HiddenPrefix", "Name", "Throws", "Hits", "Accuracy"]);
    }
    for accuracy in &result_vec {
        accuracy.serialize(&mut wtr, &fileprefix);
    }
    wtr.flush();
}



pub fn print_side_information(games: &Vec<Game>) {
    let data = calc_side_information(games);
    println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
    println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", data.left.wins, data.left.points, data.left.hits as f32 / data.left.throws as f32 * 100.0, data.left.hits, data.left.throws, data.left.schluck, data.left.beer);
    println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", data.right.wins, data.right.points, data.right.hits as f32 / data.right.throws as f32 * 100.0, data.right.hits, data.right.throws, data.right.schluck, data.right.beer);
    println!();
}
pub fn csv_side_information(games: &Vec<Game>, file_prefix: &String, date :&String){
    let data = calc_side_information(games);
    let (mut wtr, file_exists) = open_writer(date.to_string()+"side_information.csv");
    if !file_exists{
        wtr.write_record(&["HiddenPrefix", "Side", "Wins", "Points", "Hits", "Throws", "Percentage", "Strafschluck", "Strafbier"]);
    }
    data.serialize(&mut wtr, file_prefix);
}
