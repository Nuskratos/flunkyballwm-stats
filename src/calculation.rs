use std::collections::HashMap;
use crate::data::{Game, Team, TeamMember, AdditionalType};

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
                        }else{
                            schluck_right += 1;
                        }
                    }
                    AdditionalType::STRAFBIER => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            beer_left += 1;
                        }else{
                            beer_right += 1;
                        }}
                    _ => {}
                }
            }
        }
    }
    println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
    println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", left_wins, left_points, left_hits as f32 / left_throws as f32 * 100.0, left_hits, left_throws, schluck_left, beer_left);
    println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", right_wins, right_points, right_hits as f32 / right_throws as f32 * 100.0, right_hits, right_throws, schluck_right, beer_right);
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