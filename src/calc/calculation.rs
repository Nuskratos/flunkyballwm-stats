use std::collections::HashMap;
use crate::calc::accuracy_data::{Accuracy, print_accuracy};
use crate::calc::chain_calc::calculate_hit_and_miss_chains_team_player;
use crate::calc::drink_total_data::PlayerDrinkingSpeed;
use crate::calc::penalties_calc::calculate_amount_of_penalties;
use crate::calc::ppg_calc::calculate_amount_of_points_per_game;
use crate::calc::side_information_calc::calc_side_information;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::throw_per_game_calc::calculate_throws_per_game;
use crate::data::{Game, Team, TeamMember};
use crate::util::{name_from_id, player_name_from_id, print_line_break, team_from_player, team_id_from_player, team_name_from_id};
use crate::team_player_data::NAME_WIDTH; // Used in printlines

pub fn percentage(divisor: usize, divident: usize) -> f32 { divisor as f32 / divident as f32 * 100.0 }

pub fn average(divisor: u32, dividend: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average(dividend: u32, divisor: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average_f(divisor: u32, dividend: f32) -> f32 { dividend / divisor as f32 }

pub fn print_amount_of_points_per_game(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let (team_vec, player_vec) = calculate_amount_of_points_per_game(games);
    let width = 9;
    let total_line_width = 55;
    println!("Points per Game:");
    println!("While the points per Game for a team are being a solid representation, the PpG for a Person should be taken with a grain of salt!\n\
    It is calculated, that the points are split if both finished in the same round, and that they split the 2 points for a win if they finish in different rounds.\n\
    Being in a Team with a fast drinker will significantly reduce this metric, and throwing accuracy is completely ignored!");
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} |", "Name", "Points", "PpG");
    print_line_break(total_line_width);
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$.2} |", team_name_from_id(team.0, teams), team.1.points, team.1.ppg());
    }
    print_line_break(total_line_width);
    for player in player_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$.2} |", player_name_from_id(player.0, players), player.1.points, player.1.ppg());
    }
    println!();
}


pub fn print_amount_of_penalties(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let (team_vec, player_vec) = calculate_amount_of_penalties(games, teams, players);
    let width = 12;
    let total_line_width = 91;
    println!("Penalties:");
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Name", "Strafschluck", "Strafbeer", "SpG", "BpG");
    print_line_break(total_line_width);
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} | {:>width$.2} |", team_name_from_id(team.0, teams), team.1.schlucke, team.1.beers, team.1.spg(), team.1.bpg());
    }
    print_line_break(total_line_width);
    for player in player_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} | {:>width$.2} |", player_name_from_id(player.0, players), player.1.schlucke, player.1.beers, player.1.spg(), player.1.bpg());
    }
    println!();
}

pub fn print_hit_and_miss_chains(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let (team_vec, player_vec) = calculate_hit_and_miss_chains_team_player(games, teams);
    let width = 11;
    let total_line_width = 59;
    println!("Hit and miss chains");
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} |", "Name", "Hit-chain", "Miss-chain");
    print_line_break(total_line_width);
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", team_name_from_id(team.0, teams), team.1.total_hit, team.1.total_miss);
    }
    print_line_break(total_line_width);
    for player in player_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", player_name_from_id(player.0, players), player.1.total_hit, player.1.total_miss);
    }
    println!();
}

pub fn print_enemy_accuracy(games: &Vec<Game>, teams: &Vec<Team>) {
    let mut enemy_accuracy: HashMap<&Team, Accuracy> = HashMap::new();
    for game in games {
        let first_enemy_stats = team_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let second_enemy_stats = if &game.left_team == first_enemy_stats { &game.right_team } else { &game.left_team };
        for (ix, round) in game.rounds.iter().enumerate() {
            let passive_team = if ix % 2 == 0 { second_enemy_stats } else { first_enemy_stats };
            enemy_accuracy.entry(passive_team).and_modify(|x| x.add_throw(round.hit)).or_insert(Accuracy { name: passive_team.name, hits: if round.hit { 1 } else { 0 }, throws: 1 });
        }
    }
    let mut acc_vec: Vec<(&Team, Accuracy)> = Vec::new();
    for team in enemy_accuracy {
        acc_vec.push((team.0, team.1));
    }
    acc_vec.sort_by(|a, b| a.1.percentage().partial_cmp(&b.1.percentage()).unwrap());
    println!("Enemy Accuracy:");
    let width = 10;
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Teamname", "Throws", "Hits", "Percentage");
    for team in acc_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team.0.name, team.1.throws, team.1.hits, team.1.percentage());
    }
    println!();
}

pub fn print_average_throws_per_game(games: &Vec<Game>, teams: &Vec<Team>, players:&Vec<TeamMember>) {
    let data = calculate_throws_per_game(games, teams);
    println!("Average throws per game:");
    let width = 10;
    let total_width = 70;
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Team", "Games", "Throws", "Average");
    print_line_break(total_width);
    println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", "Total", games.len(), data.total_throws, data.total_throws as f32/games.len() as f32);
    print_line_break(total_width);
    for team in data.team {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team_name_from_id(team.0, teams), team.1.games, team.1.throws, wrong_way_average(team.1.games, team.1.throws));
    }
    print_line_break(total_width);
    for player in data.player{
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", player_name_from_id(player.0, players), player.1.games, player.1.throws, wrong_way_average(player.1.games, player.1.throws));
    }
    println!();
}


pub fn print_throwing_accuracy(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let mut throws = 0;
    let mut hits = 0;
    let mut player_scores = HashMap::new();
    let mut team_scores = HashMap::new();

    for game in games {
        for round in &game.rounds {
            let (player_throws, player_hits) = player_scores.entry(round.thrower.id).or_insert((0, 0));
            let (team_throws, team_hits) = team_scores.entry(team_id_from_player(round.thrower.id, teams)).or_insert((0, 0));
            throws = throws + 1;
            *player_throws += 1;
            *team_throws += 1;
            if round.hit {
                hits = hits + 1;
                *player_hits += 1;
                *team_hits += 1;
            }
        }
    }
    println!("{:<30} threw: {} and hit: {} which is {:4.2}%", "Overall", throws, hits, hits as f32 / throws as f32 * 100.0);
    print_line_break(70);

    let mut result_team_vec: Vec<Accuracy> = Vec::new();
    for score in team_scores {
        result_team_vec.push(Accuracy { throws: score.1.0, hits: score.1.1, name: name_from_id(score.0, teams, players) });
    }
    result_team_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_team_vec.reverse();
    for accuracy in &result_team_vec {
        print_accuracy(accuracy);
    }
    print_line_break(70);
    let mut result_vec: Vec<Accuracy> = Vec::new();
    for score in player_scores {
        result_vec.push(Accuracy { throws: score.1.0, hits: score.1.1, name: name_from_id(score.0, teams, players) });
    }
    result_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_vec.reverse();
    for accuracy in &result_vec {
        print_accuracy(accuracy);
    }
    print_line_break(70);
    println!();
}


pub fn print_side_information(games: &Vec<Game>) {
    let data = calc_side_information(games);
    println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
    println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", data.left.wins, data.left.points, data.left.hits as f32 / data.left.throws as f32 * 100.0, data.left.hits, data.left.throws, data.left.schluck, data.left.beer);
    println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", data.right.wins, data.right.points, data.right.hits as f32 / data.right.throws as f32 * 100.0, data.right.hits, data.right.throws, data.right.schluck, data.right.beer);
    println!();
}


pub fn print_team_first_throws(games: &Vec<Game>, teams: &Vec<Team>) {
    // Times going first, times won going first, times going second, times won going second
    let mut first_throws: HashMap<u32, (u32, u32, u32, u32)> = HashMap::new();
    for game in games {
        let team_id_going_first = team_id_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let (ffirst, fwon, _, _) = first_throws.entry(team_id_going_first).or_insert((0, 0, 0, 0));
        *ffirst += 1;
        if team_id_going_first == game.winning_team_id() {
            *fwon += 1;
        } else {}
    }
    for game in games { // duplicate because of 2nd mutable borrow in first_throws.entry TODO make prettier
        let team_id_going_second = team_id_from_player(game.rounds.first().unwrap().runner.id, teams);
        let (_, _, ssecond, sw_second) = first_throws.entry(team_id_going_second).or_insert((0, 0, 0, 0));
        *ssecond += 1;
        if team_id_going_second == game.winning_team_id() {
            *sw_second += 1;
        }
    }

    let mut result_team_vec: Vec<(u32, (u32, u32, u32, u32))> = Vec::new();
    for team_info in first_throws {
        result_team_vec.push(team_info);
    }
    result_team_vec.sort_by(|a, b| a.1.0.cmp(&b.1.0));
    result_team_vec.reverse();
    println!("Teamname:                   | Going first | Won as first | Going Second | Won as Second");
    for elem in result_team_vec {
        println!("{:<27} | {:<11} | {:<12} | {:<12} | {:<8}", team_name_from_id(elem.0, teams), elem.1.0, elem.1.1, elem.1.2, elem.1.3);
    }
    println!();
}

pub fn print_first_throw_effect(games: &Vec<Game>) {
    let mut amount_first_throw_win = 0;
    let mut amount_first_hit = 0;
    let mut amount_first_hit_win = 0;
    for game in games {
        let mut first_hit = false;
        if game.rounds.first().unwrap().hit {
            amount_first_hit += 1;
            first_hit = true;
        }
        let mut winning_ids = (0, 0);
        if game.result.points_left > game.result.points_right {
            winning_ids = (game.left_1.id, game.left_2.id);
        } else {
            winning_ids = (game.right_1.id, game.right_2.id);
        }
        let thrower_id = game.rounds.first().unwrap().thrower.id;
        if thrower_id == winning_ids.0 || thrower_id == winning_ids.1 {
            amount_first_throw_win += 1;
            if first_hit {
                amount_first_hit_win += 1;
            }
        }
    }
    let amount_first_miss = games.len() - amount_first_hit;
    let amount_first_miss_win = amount_first_throw_win - amount_first_hit_win;
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht {} mal gewonnen. Das sind {:.1}%", games.len(), amount_first_throw_win, percentage(amount_first_throw_win, games.len()));
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst getroffen. Dabei {} mal gewonnen. Das sind {:.1}%", amount_first_hit, amount_first_hit_win, percentage(amount_first_hit_win, amount_first_hit));
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst verfehlt. Dabei {} mal gewonnen. Das sind {:.1}%", amount_first_miss, amount_first_miss_win, percentage(amount_first_miss_win, amount_first_miss));
    println!();
}