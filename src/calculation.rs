use std::collections::HashMap;
use crate::data::{Game, Team, TeamMember};

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
fn print_accuracy(accuracy: &Accuracy){
    println!("{:<30} threw: {:>3} and hit: {:>3} which is {:<4.2}%", accuracy.name, accuracy.throws, accuracy.hits, accuracy.percentage());
}
fn print_line_break(){
    println!("{:-<70}", "-")
}