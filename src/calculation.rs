use std::collections::HashMap;
use crate::data::{Game, Team, TeamMember};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::team_player_data::*;

pub fn percentage(divisor: usize, divident: usize) -> f32 { divisor as f32 / divident as f32 * 100.0 }

pub fn average(divisor: u32, dividend: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average(dividend: u32, divisor: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average_f(divisor: u32, dividend: f32) -> f32 { dividend / divisor as f32 }


pub fn print_strafschluck_effect(games: &Vec<Game>, teams: &Vec<Team>) {
    let data = calculate_strafschluck(games, teams);
    let clean_average = average(data.clean_hits, data.clean_drinks);
    let straf_average = average(data.straf_hits, data.straf_drinks);
    let diff_average = clean_average - straf_average;
    let straf_per_finished = data.straf_schluecke as f32 / data.straf_drinks as f32;
    println!("Calculated Strafschluck Data:");
    println!("Clean: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", data.clean_drinks, data.clean_hits, clean_average);
    println!("Straf: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", data.straf_drinks, data.straf_hits, straf_average);
    println!("Effect of {} Strafschlucke: {:.3}\tNormalized for 1 Strafschluck per finished drink {:.3}", data.straf_schluecke, diff_average, diff_average / straf_per_finished);
    // Strafschluecke should be calculated as per drink finished ( 3 Strafschluecke on 2 drinks -> 1.5 to check for the average effect of a single one
}

#[derive(Default)]
struct StrafschluckData {
    clean_hits: u32,
    clean_drinks: u32,
    straf_hits: u32,
    straf_drinks: u32,
    straf_schluecke: u32,
}

fn calculate_strafschluck(games: &Vec<Game>, teams: &Vec<Team>) -> StrafschluckData {
    let mut data : StrafschluckData = Default::default();
    for game in games {
        let mut first_hits = 0;
        let mut second_hits = 0;
        let mut first_schluck_drank = 0;
        let mut second_schluck_drank = 0;
        let mut first_clean = true;
        let mut second_clean = true;
        let first_team = team_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let mut strafbeer_hit: HashMap<u32, u32> = HashMap::new();
        for (ix, round) in game.rounds.iter().enumerate() {
            if round.hit {
                if ix % 2 == 0 {
                    first_hits += 1;
                } else {
                    second_hits += 1;
                }
            }
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => if player_in_team(add.source.id, first_team) {
                        second_schluck_drank += 1;
                        second_clean = false;
                    } else {
                        first_schluck_drank += 1;
                        first_clean = false;
                    },
                    STRAFBIER => { // TODO REALLY extract this into a function
                        if player_in_team(add.source.id, first_team) {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, first_hits, &add.source.id);
                            strafbeer_hit.entry(add.source.id).and_modify(|x| *x = first_hits).or_insert(first_hits);
                            data.add_finished_beer(first_clean, hits_for_this_beer, first_schluck_drank, 1);
                        } else {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, second_hits, &add.source.id);
                            strafbeer_hit.entry(add.source.id).and_modify(|x| *x = second_hits).or_insert(second_hits);
                            data.add_finished_beer(second_clean, hits_for_this_beer, second_schluck_drank, 1);
                        }
                    }
                    FINISHED => {
                        if player_in_team(add.source.id, first_team) { // Duplicate as above only without the + 1 for hits
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, first_hits, &add.source.id);
                            data.add_finished_beer(first_clean, hits_for_this_beer, first_schluck_drank, 0);
                        } else {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, second_hits, &add.source.id);
                            data.add_finished_beer(second_clean, hits_for_this_beer, second_schluck_drank, 0);
                        }
                    }
                }
            }
        }
    }
    data
}
fn hits_used_for_this_beer(strafbeer_hit : &HashMap<u32, u32>, hit_amount : u32, key : &u32) -> u32{
    if strafbeer_hit.contains_key(key) {
        hit_amount - strafbeer_hit.get(key).unwrap()
    }else {
        hit_amount
    }
}
impl StrafschluckData {
    pub fn add_finished_beer(&mut self, clean : bool, hits_for_this_beer : u32, strafschluck_drank : u32, additional_hit_from_strafbeer : u32){
        if clean {
            self.clean_drinks +=1;
            self.clean_hits += hits_for_this_beer + additional_hit_from_strafbeer;
        }else {
            self.straf_drinks +=1;
            self.straf_hits += hits_for_this_beer + additional_hit_from_strafbeer;
            self.straf_schluecke += strafschluck_drank;
        }
    }
    //pub fn beer_emptied(&mut self, player_id : u32, first_team : &Team, strafbeer_hit : &HashMap<u32, u32>){

//    }
}

#[derive(Default)]
pub struct ChainInformation {
    current_hit: u32,
    current_miss: u32,
    total_hit: u32,
    total_miss: u32,
}

impl ChainInformation {
    pub fn create(hit: bool) -> ChainInformation {
        let mut ret: ChainInformation = Default::default();
        ret.throw(hit);
        ret
    }
    pub fn throw(&mut self, hit: bool) {
        if hit {
            self.hit();
        } else {
            self.miss();
        }
    }
    fn hit(&mut self) {
        self.current_hit += 1;
        if self.current_hit > self.total_hit {
            self.total_hit = self.current_hit;
        }
        self.current_miss = 0;
    }
    fn miss(&mut self) {
        self.current_miss += 1;
        if self.current_miss > self.total_miss {
            self.total_miss = self.current_miss;
        }
        self.current_hit = 0;
    }
}

pub fn print_hit_and_miss_chains(games: &Vec<Game>, teams: &Vec<Team>) {
    let mut team_chain: HashMap<&Team, ChainInformation> = HashMap::new();
    let mut player_chain: HashMap<&TeamMember, ChainInformation> = HashMap::new();
    for game in games {
        for round in &game.rounds {
            let team = team_from_player(round.thrower.id, teams);
            team_chain.entry(team).and_modify(|x| x.throw(round.hit)).or_insert(ChainInformation::create(round.hit));
            player_chain.entry(&round.thrower).and_modify(|x| x.throw(round.hit)).or_insert(ChainInformation::create(round.hit));
        }
    }
    let width = 11;
    let total_line_width = 59;
    println!("Hit and miss chains");
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} |", "Name", "Hit-chain", "Miss-chain");
    print_line_break(total_line_width);
    let mut team_vec: Vec<(&Team, ChainInformation)> = Vec::new();
    for team in team_chain {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.total_hit.partial_cmp(&b.1.total_hit).unwrap());
    team_vec.reverse();
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", team.0.name, team.1.total_hit, team.1.total_miss);
    }


    print_line_break(total_line_width);
    let mut player_vec: Vec<(&TeamMember, ChainInformation)> = Vec::new();
    for player in player_chain {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.total_hit.partial_cmp(&b.1.total_hit).unwrap());
    player_vec.reverse();
    for player in player_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", player.0.name, player.1.total_hit, player.1.total_miss);
    }
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
}

pub struct TeamThrowGames {
    throws: u32,
    games: u32,
}

impl TeamThrowGames {
    pub fn add_throws(&mut self, throws: u32) {
        self.throws += throws;
        self.games += 1;
    }
    pub fn new(throws: u32) -> TeamThrowGames {
        TeamThrowGames {
            throws,
            games: 1,
        }
    }
}

pub fn print_average_throws(games: &Vec<Game>, teams: &Vec<Team>) {
    let mut total_throws: u32 = 0;
    let mut team_throws: HashMap<&Team, TeamThrowGames> = HashMap::new();
    for game in games {
        total_throws += game.rounds.len() as u32;
        let throws_of_second_team = game.rounds.len() as u32 / 2;
        if game.rounds.len() % 2 == 0 { // Both teams have the same number of throws
            team_throws.entry(&game.left_team).and_modify(|x| x.add_throws(throws_of_second_team)).or_insert(TeamThrowGames::new(throws_of_second_team));
            team_throws.entry(&game.right_team).and_modify(|x| x.add_throws(throws_of_second_team)).or_insert(TeamThrowGames::new(throws_of_second_team));
        } else { // Opening team had more throws
            let opening_team = team_from_player(game.rounds.first().unwrap().thrower.id, teams);
            if &game.left_team == opening_team {
                team_throws.entry(&game.left_team).and_modify(|x| x.add_throws(throws_of_second_team + 1)).or_insert(TeamThrowGames::new(throws_of_second_team + 1));
                team_throws.entry(&game.right_team).and_modify(|x| x.add_throws(throws_of_second_team)).or_insert(TeamThrowGames::new(throws_of_second_team));
            } else {
                team_throws.entry(&game.right_team).and_modify(|x| x.add_throws(throws_of_second_team + 1)).or_insert(TeamThrowGames::new(throws_of_second_team + 1));
                team_throws.entry(&game.left_team).and_modify(|x| x.add_throws(throws_of_second_team)).or_insert(TeamThrowGames::new(throws_of_second_team));
            }
        }
    }
    println!("Average throws: Games: {:>3}\tRounds: {:>4}\tAverage: {:>2.2}", games.len(), total_throws, wrong_way_average(games.len() as u32, total_throws));
    let mut team_vec: Vec<(&Team, TeamThrowGames)> = Vec::new();
    for team in team_throws {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| wrong_way_average(a.1.games, a.1.throws).partial_cmp(&wrong_way_average(b.1.games, b.1.throws)).unwrap());
    let width = 10;
    println!("| {:^NAME_WIDTH$} | {:^width$} | {:^width$} | {:^width$} |", "Team", "Games", "Throws", "Average");
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} | {:>width$.2} |", team.0.name, team.1.games, team.1.throws, wrong_way_average(team.1.games, team.1.throws));
    }
}

pub fn player_in_game(game: &Game, player: &TeamMember) -> bool {
    let player_ids = vec![game.left_1.id, game.left_2.id, game.right_1.id, game.right_2.id];
    return player_ids.contains(&player.id);
}

pub fn print_complete_drinking_speed(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32) {
    println!("Different drinking speed metrics:
Pure finished: Finished drinks without StrafSchluck
Pure average: Finished drinks, not-finished rounds with >=flat(Pure) count as (rounds+1) - no StrafSchluck
All finished: Finished drinks with Strafschluck
All average: Finished drinks, not-finished with (rounds >=flat(All finished)) count as (rounds+1) including Strafschlucks
for all above: StrafBeer counts as finished +1");
    println!("Selected Strafschluck effect: {:.2} rounds", schluck_effect);
    let mut playerspeeds: Vec<(PlayerFinishedStats, PlayerAvgStats, &TeamMember)> = Vec::new();
    for player in players {
        let finished = calculate_finished(games, player, teams, schluck_effect);
        let averages = calculate_avg(games, player, teams, &finished, schluck_effect);
        playerspeeds.push((finished, averages, player));
    }
    playerspeeds.sort_by(|a, b| wrong_way_average_f(a.1.all_avg.0, a.1.all_avg.1).partial_cmp(&wrong_way_average_f(b.1.all_avg.0, b.1.all_avg.1)).unwrap());
    let n_c = 10;
    let width = 17;
    println!("{:-<94}", "-");
    println!("| {:^n_c$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Player", "Pure finished", "Pure average", "All finished", "All average");
    println!("{:-<94}", "-");
    for player in playerspeeds {
        let pure_finished = format!("{:>.2} ({:>4} / {:>2})", wrong_way_average(player.0.pure_finished.0, player.0.pure_finished.1), player.0.pure_finished.1, player.0.pure_finished.0);
        let pure_average = format!("{:>.2} ({:>4} / {:>2})", wrong_way_average(player.1.pure_avg.0, player.1.pure_avg.1), player.1.pure_avg.1, player.1.pure_avg.0);
        let all_finished = format!("{:>.2} ({:>4} / {:>2})", wrong_way_average_f(player.0.all_finished.0, player.0.all_finished.1), player.0.all_finished.1, player.0.all_finished.0);
        let all_average = format!("{:>.2} ({:>4} / {:>2})", wrong_way_average_f(player.1.all_avg.0, player.1.all_avg.1), player.1.all_avg.1, player.1.all_avg.0);
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

pub fn calculate_avg(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, finished_stats: &PlayerFinishedStats, schluck_effect: f32) -> PlayerAvgStats {
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
                    tmp_schluck = 0.0;
                    tmp_round = 0;
                }
                if add.kind == STRAFSCHLUCK && team_id_from_player(add.source.id, teams) != player_team {
                    tmp_schluck += schluck_effect;
                    schluck_happened = true;
                }
            }
            if i == game.rounds.len() - 1 && !person_finished {
                let pure_finished_average = wrong_way_average(finished_stats.pure_finished.0, finished_stats.pure_finished.1).floor() as u32;
                if tmp_round >= pure_finished_average && pure_finished_average > 0 {
                    avg_stats.p_avg(1, tmp_round + 1)
                }
                let all_finished_average = wrong_way_average_f(finished_stats.all_finished.0, finished_stats.all_finished.1).floor();
                if tmp_round as f32 + tmp_schluck >= all_finished_average && all_finished_average > 0.0 {
                    avg_stats.a_avg(1, tmp_round as f32 + tmp_schluck + 1.0);
                }
            }
        }
    }
    avg_stats
}

pub fn calculate_finished(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, schluck_effect: f32) -> PlayerFinishedStats {
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
                    tmp_round = 0;
                    tmp_schluck = 0.0;
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
                    STRAFSCHLUCK => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            schluck_left += 1;
                        } else {
                            schluck_right += 1;
                        }
                    }
                    STRAFBIER => {
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
    fn add_throw(&mut self, hit: bool) {
        self.throws += 1;
        if hit {
            self.hits += 1;
        }
    }
}

fn player_in_team(player_id: u32, team: &Team) -> bool {
    return team.member_1.id == player_id || team.member_2.id == player_id;
}

fn team_from_player(player_id: u32, teams: &Vec<Team>) -> &Team {
    for team in teams {
        if team.member_1.id == player_id || team.member_2.id == player_id {
            return team;
        }
    }
    &TEAM_INVALID
}

fn team_id_from_player(player_id: u32, teams: &Vec<Team>) -> u32 {
    team_from_player(player_id, teams).id
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

fn print_line_break(width: usize) {
    println!("{:-<width$}", "-")
}