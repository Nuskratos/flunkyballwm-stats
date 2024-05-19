use std::backtrace::BacktraceStatus;
use std::collections::HashMap;
use crate::data::{Game, Team, TeamMember, AdditionalType};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};

pub fn percentage(first: usize, second: usize) -> f32 {
    first as f32 / second as f32 * 100.0
}

pub fn average(beers: u32, rounds: u32) -> f32 { rounds as f32 / beers as f32 }

pub fn average_f(beers: u32, rounds: f32) -> f32 { rounds / beers as f32 }


pub fn player_in_game(game: &Game, player: &TeamMember) -> bool {
    let player_ids = vec![game.left_1.id, game.left_2.id, game.right_1.id, game.right_2.id];
    return player_ids.contains(&player.id);
}

pub fn print_complete_drinking_speed(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32) {
    println!("Pure finished: Finished drinks without StrafSchluck
Pure average: Finished drinks, not-finished rounds with >=flat(Pure) count as (rounds+1) - no StrafSchluck
All finished: Finished drinks with Strafschluck
All average: Finished drinks, not-finished with (rounds >=flat(All finished)) count as (rounds+1) including Strafschlucks
for all above: StrafBeer counts as finished +1");
    let mut playerspeeds : Vec<(PlayerFinishedStats, PlayerAvgStats, &TeamMember)> = Vec::new();
    for player in players {
        let finisheds = calculate_finished(games, player, teams, schluck_effect);
        let averages = calculate_avg(games, player, teams, &finisheds, schluck_effect);
        playerspeeds.push((finisheds, averages, player));
    }
    playerspeeds.sort_by(|a, b| average_f(a.1.all_avg.0, a.1.all_avg.1).partial_cmp(&average_f(b.1.all_avg.0, b.1.all_avg.1)).unwrap());
    let n_c = 10;
    let width = 17;
    println!("{:-<94}", "-");
    println!("| {:^n_c$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Player", "Pure finished", "Pure average", "All finished", "All average");
    println!("{:-<94}", "-");
    for player in playerspeeds {
        let pure_finished = format!("{:>.2} ({:>4} / {:>2})", average(player.0.pure_finished.0, player.0.pure_finished.1), player.0.pure_finished.1, player.0.pure_finished.0);
        let pure_average = format!("{:>.2} ({:>4} / {:>2})", average(player.1.pure_avg.0, player.1.pure_avg.1), player.1.pure_avg.1, player.1.pure_avg.0);
        let all_finished = format!("{:>.2} ({:>4} / {:>2})", average_f(player.0.all_finished.0, player.0.all_finished.1), player.0.all_finished.1, player.0.all_finished.0);
        let all_average = format!("{:>.2} ({:>4} / {:>2})", average_f(player.1.all_avg.0, player.1.all_avg.1), player.1.all_avg.1, player.1.all_avg.0);
        println!("| {:>n_c$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |", player.2.name, pure_finished, pure_average, all_finished, all_average);
    }
}

pub struct PlayerAvgStats {
    pure_avg: (u32, u32),
    all_avg: (u32, f32),
}

impl PlayerAvgStats {
    pub fn new() -> PlayerAvgStats {
        PlayerAvgStats {
            pure_avg: (0, 0),
            all_avg: (0, 0.0),
        }
    }
    pub fn p_avg(&mut self, beers: u32, rounds: u32) {
        self.pure_avg.0 += beers;
        self.pure_avg.1 += rounds;
    }
    pub fn a_avg(&mut self, beers: u32, rounds: f32) {
        self.all_avg.0 += beers;
        self.all_avg.1 += rounds;
    }
}

// First is always the amount of beers, second is amound of rounds
pub struct PlayerFinishedStats {
    pure_finished: (u32, u32),
    all_finished: (u32, f32),
}

impl PlayerFinishedStats {
    pub fn new() -> PlayerFinishedStats {
        PlayerFinishedStats {
            pure_finished: (0, 0),
            all_finished: (0, 0.0),
        }
    }
    pub fn p_finished(&mut self, beers: u32, rounds: u32) {
        self.pure_finished.0 += beers;
        self.pure_finished.1 += rounds;
    }
    pub fn a_finished(&mut self, beers: u32, rounds: f32) {
        self.all_finished.0 += beers;
        self.all_finished.1 += rounds;
    }
}

pub fn calculate_avg(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, finished_stats: &PlayerFinishedStats, schluck_effect : f32) -> PlayerAvgStats {
    let mut avg_stats = PlayerAvgStats::new();
    for game in games {
        if !player_in_game(game, player) {
            continue;
        }
        let player_team = team_id_from_player(player.id, teams);
        let mut tmp_round = 0;
        let mut tmp_schluck = 0.0;
        let is_from_first_team = player_team == team_id_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let offset = if is_from_first_team { 0 } else { 1 };
        let mut schluck_happened = false;
        let mut person_finished = false;
        for (i, round) in game.rounds.iter().enumerate() {
            if i % 2 == offset && round.hit { // correct team hitting
                tmp_round += 1;
            }
            for add in &round.additionals {
                if add.kind == FINISHED && add.source.id == player.id {
                    if !schluck_happened {
                        avg_stats.p_avg(1, tmp_round);
                    }
                    avg_stats.a_avg(1, tmp_round as f32 + tmp_schluck);
                    person_finished = true;
                    // Some kind of exit to game would be efficient, but should not have consequences, because nothing will be added
                }
                if add.kind == STRAFBIER && add.source.id == player.id {
                    if !schluck_happened {
                        avg_stats.p_avg(1, tmp_round + 1);
                    }
                    avg_stats.a_avg(1, tmp_round as f32 + tmp_schluck + 1.0);
                    // continuing in case the strafbier was finished
                }
                if add.kind == STRAFSCHLUCK && team_id_from_player(add.source.id, teams) != player_team {
                    tmp_schluck += schluck_effect;
                    schluck_happened = true;
                }
            }
            if i == game.rounds.len() - 1 && !person_finished{
                if tmp_round >= average(finished_stats.pure_finished.0, finished_stats.pure_finished.1).floor() as u32 {
                    avg_stats.p_avg(1, tmp_round + 1)
                }
                if tmp_round as f32 + tmp_schluck >= average_f(finished_stats.all_finished.0, finished_stats.all_finished.1).floor() {
                    avg_stats.a_avg(1, tmp_round as f32 + tmp_schluck + 1.0);
                }
            }
        }
    }
    avg_stats
}

pub fn calculate_finished(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, schluck_effect : f32) -> PlayerFinishedStats {
    let mut finshed_stats = PlayerFinishedStats::new();
    for game in games {
        if !player_in_game(game, player) {
            continue;
        }
        let player_team = team_id_from_player(player.id, teams);
        let mut tmp_round = 0;
        let mut tmp_schluck = 0.0;
        let is_from_first_team = player_team == team_id_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let offset = if is_from_first_team { 0 } else { 1 };
        let mut schluck_happened = false;
        let mut person_finished = false;
        for (i, round) in game.rounds.iter().enumerate() {
            if i % 2 == offset && round.hit { // correct team hitting
                tmp_round += 1;
            }
            for add in &round.additionals {
                if add.kind == FINISHED && add.source.id == player.id {
                    if !schluck_happened {
                        finshed_stats.p_finished(1, tmp_round);
                    }
                    finshed_stats.a_finished(1, tmp_round as f32 + tmp_schluck);
                    person_finished = true;
                    // Some kind of exit to game would be efficient, but should not have consequences, because nothing will be added
                }
                if add.kind == STRAFBIER && add.source.id == player.id {
                    if !schluck_happened {
                        finshed_stats.p_finished(1, tmp_round + 1);
                    }
                    finshed_stats.a_finished(1, tmp_round as f32 + tmp_schluck + 1.0);
                    // continuing in case the strafbier was finished
                }
                if add.kind == STRAFSCHLUCK && team_id_from_player(add.source.id, teams) != player_team {
                    tmp_schluck += schluck_effect;
                    schluck_happened = true;
                }
            }
        }
    }
    finshed_stats
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
                if round.hit {
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
        if team.member_1.id == playerid || team.member_2.id == playerid {
            return team.id;
        }
    }
    0
}

fn team_name_from_id(team_id: u32, teams: &Vec<Team>) -> &str {
    for team in teams {
        if team.id == team_id {
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