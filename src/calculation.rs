use std::collections::HashMap;
use crate::data::{Game, Team, TeamMember, AdditionalType};

pub fn percentage(first: usize, second: usize) -> f32 {
    first as f32 / second as f32 * 100.0
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
    print_line_break();

    let mut result_team_vec: Vec<Accuracy> = Vec::new();
    for score in team_scores {
        result_team_vec.push(Accuracy { throws: score.1.0, hits: score.1.1, name: name_from_id(score.0, teams, players) });
    }
    result_team_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_team_vec.reverse();
    for accuracy in &result_team_vec {
        print_accuracy(accuracy);
    }
    print_line_break();
    let mut result_vec: Vec<Accuracy> = Vec::new();
    for score in player_scores {
        result_vec.push(Accuracy { throws: score.1.0, hits: score.1.1, name: name_from_id(score.0, teams, players) });
    }
    result_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_vec.reverse();
    for accuracy in &result_vec {
        print_accuracy(accuracy);
    }
    print_line_break();
}

pub fn print_side_information(games: &Vec<Game>) {
    //TODO maybe extract all those values into a hashmap for easier readability?
    let mut left_wins = 0;
    let mut right_wins = 0;
    let mut left_points = 0;
    let mut right_points = 0;
    let mut left_hits = 0;
    let mut right_hits = 0;
    let mut left_throws = 0;
    let mut right_throws = 0;
    let mut throws = 0;
    let mut schluck_right = 0;
    let mut schluck_left = 0;
    let mut beer_right = 0;
    let mut beer_left = 0;
    for game in games {
        if game.result.points_left > game.result.points_right {
            left_wins += 1;
        } else {
            right_wins += 1;
        }
        left_points += game.result.points_left;
        right_points += game.result.points_right;
        for round in &game.rounds {
            throws += 1;
            if round.thrower.id == game.left_1.id || round.thrower.id == game.left_2.id {
                left_throws += 1;
                if (round.hit) {
                    left_hits += 1;
                }
            } else {
                right_throws += 1;
                if round.hit {
                    right_hits += 1;
                }
            }
            for additional in &round.additionals {
                match &additional.kind {
                    AdditionalType::STRAFSCHLUCK => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            schluck_left += 1;
                        } else {
                            schluck_right += 1;
                        }
                    }
                    AdditionalType::STRAFBIER => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            beer_left += 1;
                        } else {
                            beer_right += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
    println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", left_wins, left_points, left_hits as f32 / left_throws as f32 * 100.0, left_hits, left_throws, schluck_left, beer_left);
    println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", right_wins, right_points, right_hits as f32 / right_throws as f32 * 100.0, right_hits, right_throws, schluck_right, beer_right);
}


pub fn print_team_first_throws(games: &Vec<Game>, teams :&Vec<Team>){
    // Times going first, times won going first, times going second, times won going second
    let mut first_throws :HashMap<u32, (u32, u32, u32, u32)> = HashMap::new();
    for game in games{
        let team_id_going_first = team_id_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let team_id_going_second = team_id_from_player(game.rounds.first().unwrap().runner.id, teams);
        let (ffirst, fwon, fsecond, fw_second) = first_throws.entry(team_id_going_first).or_insert((0,0,0,0));
        *ffirst +=1;
        if team_id_going_first == game.winning_team_id(){
            *fwon+=1;
        }else{
        }
    }
    for game in games{ // duplicate because of 2nd mutable borrow in first_throws.entry TODO make prettier
        let team_id_going_first = team_id_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let team_id_going_second = team_id_from_player(game.rounds.first().unwrap().runner.id, teams);
        let (sfirst, swon, ssecond, sw_second) = first_throws.entry(team_id_going_second).or_insert((0,0,0,0));
        *ssecond+=1;
        if team_id_going_first == game.winning_team_id(){
        }else{
            *sw_second+=1;
        }
    }

    let mut result_team_vec: Vec<(u32,(u32, u32, u32, u32))> = Vec::new();
    for team_info in first_throws {
        result_team_vec.push(team_info);
    }
    result_team_vec.sort_by(|a, b| a.1.0.cmp(&b.1.0) );
    result_team_vec.reverse();
    println!("Teamname:                   | Going first | Won as first | Going Second | Won as Second");
    for elem in result_team_vec {
        println!("{:<27} | {:<11} | {:<12} | {:<12} | {:<8}", team_name_from_id(elem.0, teams), elem.1.0, elem.1.1, elem.1.2, elem.1.3);
    }
}

pub fn print_first_throw_effect(games: &Vec<Game>) {
    let mut amount_first_throw_win = 0;
    for game in games {
        let mut winning_ids = (0, 0);
        if game.result.points_left > game.result.points_right {
            winning_ids = (game.left_1.id, game.left_2.id);
        }else{
            winning_ids = (game.right_1.id, game.right_2.id);
        }
        let thrower_id = game.rounds.first().unwrap().thrower.id;
        if (thrower_id == winning_ids.0 || thrower_id == winning_ids.1) {
            amount_first_throw_win += 1;
        }
    }
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht {} mal gewonnen. Das sind {}%", games.len(), amount_first_throw_win, percentage(amount_first_throw_win, games.len()));
}


struct Accuracy {
    name: &'static str,
    throws: u32,
    hits: u32,
}

impl Accuracy {
    fn percentage(&self) -> f32 {
        self.hits as f32 / self.throws as f32 * 100.0
    }
}

fn team_id_from_player(playerid: u32, teams: &Vec<Team>) -> u32 {
    for team in teams {
        if (team.member_1.id == playerid || team.member_2.id == playerid) {
            return team.id;
        }
    }
    0
}
fn team_name_from_id(team_id : u32, teams : &Vec<Team>) -> &str {
    for team in teams {
        if team.id == team_id{
            return team.name;
        }
    }
    "Not Found"
}

fn name_from_id(id: u32, teams: &Vec<Team>, players: &Vec<TeamMember>) -> &'static str {
    for team in teams {
        if team.id == id {
            return team.name;
        }
    }
    for player in players {
        if player.id == id {
            return player.name;
        }
    }
    "Name not found"
}

fn print_accuracy(accuracy: &Accuracy) {
    println!("{:<30} threw: {:>3} and hit: {:>3} which is {:<4.2}%", accuracy.name, accuracy.throws, accuracy.hits, accuracy.percentage());
}

fn print_line_break() {
    println!("{:-<70}", "-")
}