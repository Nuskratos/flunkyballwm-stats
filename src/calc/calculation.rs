use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::iter::Map;
use crate::calc::chain_data::ChainInformation;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;
use crate::calc::drink_total_data::PlayerDrinkingSpeed;
use crate::calc::penalties_data::Penalties;
use crate::calc::ppg_data::PpgHolder;
use crate::data::{ARC, Game, Team, TeamMember};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::team_player_data::*;
use crate::calc::strafschluck_data::StrafschluckData;

pub fn percentage(divisor: usize, divident: usize) -> f32 { divisor as f32 / divident as f32 * 100.0 }

pub fn average(divisor: u32, dividend: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average(dividend: u32, divisor: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average_f(divisor: u32, dividend: f32) -> f32 { dividend / divisor as f32 }

pub fn print_amount_of_points_per_game(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) {
    let (team_vec, player_vec) = calculate_amount_of_points_per_game(games, teams, players);
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
}

pub fn calculate_amount_of_points_per_game(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) -> (Vec<(u32, PpgHolder)>, Vec<(u32, PpgHolder)>) {
    let mut team_map: HashMap<u32, PpgHolder> = HashMap::new();
    let mut player_map: HashMap<u32, PpgHolder> = HashMap::new();
    for game in games {
        team_map.entry(game.left_team.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.left_team.id).and_modify(|x| x.points += game.result.points_left);
        team_map.entry(game.right_team.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        team_map.entry(game.right_team.id).and_modify(|x| x.points += game.result.points_right);
        player_map.entry(game.left_1.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.left_2.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_1.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        player_map.entry(game.right_2.id).and_modify(|x| x.games += 1).or_insert(PpgHolder::new());
        let mut round_left_finished = 0;
        let mut round_right_finished = 0;
        let mut left_already_finished = false;
        let mut right_already_finished = false;
        let mut points_vec: Vec<u32> = vec![7, 5, 3];
        for arc in game.additionals_vec() {
            if arc.additional.kind == FINISHED {
                player_map.entry(arc.additional.source.id).and_modify(|x| x.points += points_vec.first().unwrap());
                points_vec.remove(0);
                // adding points for winning (2 if in the same round as first, so they have the same, 1 to each if in a later round)
                if player_in_team(arc.additional.source.id, &game.left_team) { // TODO extract into a mutable function
                    update_player_map(&mut player_map, round_left_finished, left_already_finished, game, true, &arc);
                } else { // player in right
                    update_player_map(&mut player_map, round_right_finished, right_already_finished, game, false, &arc);
                }
                // Storing the completed round info to know when the next one finishes
                if player_in_team(arc.additional.source.id, &game.left_team) {
                    left_already_finished = true;
                    round_left_finished = arc.round_nr;
                } else {
                    right_already_finished = true;
                    round_right_finished = arc.round_nr;
                }
            }
        }
    }
    let mut team_vec: Vec<(u32, PpgHolder)> = Vec::new();
    for team in team_map {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, PpgHolder)> = Vec::new();
    for player in player_map {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    (team_vec, player_vec)
}

fn update_player_map(map: &mut HashMap<u32, PpgHolder>, round_finished: u32, partner_finished: bool, game: &Game, left_team: bool, arc: &ARC) {
    if partner_finished {
        if arc.round_nr == round_finished {
            map.entry(arc.additional.source.id).and_modify(|x| x.points += 2);
        } else {
            if left_team {
                map.entry(game.left_1.id).and_modify(|x| x.points += 1);
                map.entry(game.left_2.id).and_modify(|x| x.points += 1);
            } else {
                map.entry(game.right_1.id).and_modify(|x| x.points += 1);
                map.entry(game.right_2.id).and_modify(|x| x.points += 1);
            }
        }
    }
}


pub fn calculate_amount_of_penalties(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) -> (Vec<(u32, Penalties)>, Vec<(u32, Penalties)>) {
    let mut team_map: HashMap<u32, Penalties> = HashMap::new();
    let mut player_map: HashMap<u32, Penalties> = HashMap::new();
    // Fill maps in case someone got no penalty
    for team in teams {
        team_map.insert(team.id, Default::default());
    }
    for player in players {
        player_map.insert(player.id, Default::default());
    }
    for game in games {
        team_map.entry(game.left_team.id).and_modify(|x| x.games += 1);
        team_map.entry(game.right_team.id).and_modify(|x| x.games += 1);
        player_map.entry(game.left_1.id).and_modify(|x| x.games += 1);
        player_map.entry(game.left_2.id).and_modify(|x| x.games += 1);
        player_map.entry(game.right_1.id).and_modify(|x| x.games += 1);
        player_map.entry(game.right_2.id).and_modify(|x| x.games += 1);
        for round in &game.rounds {
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => {
                        team_map.entry(team_id_from_player(add.source.id, teams)).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                        player_map.entry(add.source.id).and_modify(|x| x.schlucke += 1).or_insert(Penalties::create_schluck());
                    }
                    STRAFBIER => {
                        team_map.entry(team_id_from_player(add.source.id, teams)).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                        player_map.entry(add.source.id).and_modify(|x| x.beers += 1).or_insert(Penalties::create_beer());
                    }
                    _ => {}
                }
            }
        }
    }
    let mut team_vec: Vec<(u32, Penalties)> = Vec::new();
    for team in team_map {
        team_vec.push((team.0, team.1));
    }
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    let mut player_vec: Vec<(u32, Penalties)> = Vec::new();
    for player in player_map {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    (team_vec, player_vec)
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
}

pub fn print_strafschluck_effect(games: &Vec<Game>, teams: &Vec<Team>) -> f32 {
    let data = calculate_strafschluck(games, teams);
    println!("Calculated Strafschluck Data:");
    println!("Clean: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", data.clean_drinks, data.clean_hits, data.clean_average());
    println!("Straf: Drinks finished: {}\tHits required: {}\tAverage: {:.3}", data.straf_drinks, data.straf_hits, data.straf_average());
    println!("Effect of {} Strafschlucke: {:.3}\tNormalized for 1 Strafschluck per finished drink {:.3}", data.straf_schluecke, data.diff_average(), data.effect_of_single_schluck());
    data.effect_of_single_schluck()
}

fn calculate_strafschluck(games: &Vec<Game>, teams: &Vec<Team>) -> StrafschluckData {
    let mut data: StrafschluckData = Default::default();
    for game in games {
        let mut first_hits = 0;
        let mut second_hits = 0;
        let mut first_schluck_drank = 0;
        let mut second_schluck_drank = 0;
        let mut first_clean = true;
        let mut second_clean = true;
        let first_team = team_from_player(game.rounds.first().unwrap().thrower.id, teams);
        let mut strafbeer_hit: HashMap<u32, u32> = HashMap::new();
        let mut strafbeer_schluck: HashMap<u32, u32> = HashMap::new();
        for (ix, round) in game.rounds.iter().enumerate() {
            if round.hit {
                if ix % 2 == 0 {
                    first_hits += 1;
                } else {
                    second_hits += 1;
                }
            }
            for add in &round.additionals { // TODO extract this into a function?
                match add.kind {
                    STRAFSCHLUCK => if player_in_team(add.source.id, first_team) {
                        second_schluck_drank += 1;
                        second_clean = false;
                    } else {
                        first_schluck_drank += 1;
                        first_clean = false;
                    },
                    STRAFBIER => {
                        if player_in_team(add.source.id, first_team) {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, first_hits, &add.source.id);
                            let schlucke_for_this_beer = schlucke_used_for_this_beer(&strafbeer_schluck, first_schluck_drank, &add.source.id);
                            strafbeer_hit.entry(add.source.id).and_modify(|x| *x = first_hits).or_insert(first_hits);
                            strafbeer_schluck.entry(add.source.id).and_modify(|x| *x = first_schluck_drank).or_insert(first_schluck_drank);
                            data.add_finished_beer(first_clean, hits_for_this_beer, schlucke_for_this_beer, 1);
                        } else {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, second_hits, &add.source.id);
                            let schlucke_for_this_beer = schlucke_used_for_this_beer(&strafbeer_schluck, second_schluck_drank, &add.source.id);
                            strafbeer_hit.entry(add.source.id).and_modify(|x| *x = second_hits).or_insert(second_hits);
                            strafbeer_schluck.entry(add.source.id).and_modify(|x| *x = second_schluck_drank).or_insert(second_schluck_drank);
                            data.add_finished_beer(second_clean, hits_for_this_beer, schlucke_for_this_beer, 1);
                        }
                    }
                    FINISHED => {
                        if player_in_team(add.source.id, first_team) { // Duplicate as above only without the + 1 for hits
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, first_hits, &add.source.id);
                            let schlucke_for_this_beer = schlucke_used_for_this_beer(&strafbeer_schluck, first_schluck_drank, &add.source.id);
                            data.add_finished_beer(first_clean, hits_for_this_beer, schlucke_for_this_beer, 0);
                        } else {
                            let hits_for_this_beer = hits_used_for_this_beer(&strafbeer_hit, second_hits, &add.source.id);
                            let schlucke_for_this_beer = schlucke_used_for_this_beer(&strafbeer_schluck, second_schluck_drank, &add.source.id);
                            data.add_finished_beer(second_clean, hits_for_this_beer, schlucke_for_this_beer, 0);
                        }
                    }
                }
            }
        }
    }
    data
}

fn hits_used_for_this_beer(strafbeer_hit: &HashMap<u32, u32>, hit_amount: u32, key: &u32) -> u32 {
    if strafbeer_hit.contains_key(key) {
        hit_amount - strafbeer_hit.get(key).unwrap()
    } else {
        hit_amount
    }
}

fn schlucke_used_for_this_beer(strafbeer_schlucke: &HashMap<u32, u32>, schlucke_amount: u32, key: &u32) -> u32 {
    if strafbeer_schlucke.contains_key(key) {
        schlucke_amount - strafbeer_schlucke.get(key).unwrap()
    } else {
        schlucke_amount
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
    team_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
    for team in team_vec {
        println!("| {:>NAME_WIDTH$} | {:>width$} | {:>width$} |", team.0.name, team.1.total_hit, team.1.total_miss);
    }


    print_line_break(total_line_width);
    let mut player_vec: Vec<(&TeamMember, ChainInformation)> = Vec::new();
    for player in player_chain {
        player_vec.push((player.0, player.1));
    }
    player_vec.sort_by(|a, b| a.1.custom_cmp(&b.1).unwrap());
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
    let mut playerspeeds: Vec<PlayerDrinkingSpeed> = Vec::new();
    for player in players {
        let finished = calculate_finished(games, player, teams, schluck_effect);
        let averages = calculate_avg(games, player, teams, &finished, schluck_effect);
        playerspeeds.push(PlayerDrinkingSpeed { drink_finished: finished, drink_avg: averages, player_name: String::from(player.name) });
    }
    playerspeeds.sort_by(|a, b| a.custom_cmp(&b).unwrap());
    let n_c = 10;
    let width = 17;
    println!("{:-<94}", "-");
    println!("| {:^n_c$} | {:^width$} | {:^width$} | {:^width$} | {:^width$} |", "Player", "Pure finished", "Pure average", "All finished", "All average");
    println!("{:-<94}", "-");
    for player in playerspeeds {
        let pure_finished = format!("{:>.2} ({:>4.2} / {:>2})", player.drink_finished.pure_speed(), player.drink_finished.pure_hits, player.drink_finished.pure_drinks);
        let pure_average = format!("{:>.2} ({:>4.2} / {:>2})", player.drink_avg.pure_speed(), player.drink_avg.pure_hits, player.drink_avg.pure_drinks);
        let all_finished = format!("{:>.2} ({:>4.2} / {:>2})", player.drink_finished.all_speed(), player.drink_finished.all_hits, player.drink_finished.all_drinks);
        let all_average = format!("{:>.2} ({:>4.2} / {:>2})", player.drink_avg.all_speed(), player.drink_avg.all_hits, player.drink_avg.all_drinks);
        println!("| {:>n_c$} | {:>width$} | {:>width$} | {:>width$} | {:>width$} |", player.player_name, pure_finished, pure_average, all_finished, all_average);
    }
}

pub fn calculate_avg(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, finished_stats: &DrinkFinishedStats, schluck_effect: f32) -> DrinkAvgStats {
    let mut avg_stats = DrinkAvgStats::new();
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
                let pure_finished_average = average(finished_stats.pure_hits, finished_stats.pure_drinks).floor() as u32;
                if tmp_round >= pure_finished_average && pure_finished_average > 0 {
                    avg_stats.p_avg(1, tmp_round + 1)
                }
                let all_finished_average = (finished_stats.all_hits / finished_stats.all_drinks as f32).floor();
                if tmp_round as f32 + tmp_schluck >= all_finished_average && all_finished_average > 0.0 {
                    avg_stats.a_avg(1, tmp_round as f32 + tmp_schluck + 1.0);
                }
            }
        }
    }
    avg_stats
}

pub fn calculate_finished(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, schluck_effect: f32) -> DrinkFinishedStats {
    let mut finshed_stats = DrinkFinishedStats::new();
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

#[derive(Default)]
pub struct SideInformation {
    pub wins: u32,
    pub points: u32,
    pub hits: u32,
    pub throws: u32,
    pub schluck: u32,
    pub beer: u32,
}

pub fn print_side_information(games: &Vec<Game>) {
    let mut left: SideInformation = Default::default();
    let mut right: SideInformation = Default::default();
    for game in games {
        if game.result.points_left > game.result.points_right {
            left.wins += 1;
        } else {
            right.wins += 1;
        }
        left.points += game.result.points_left;
        right.points += game.result.points_right;
        for round in &game.rounds {
            if round.thrower.id == game.left_1.id || round.thrower.id == game.left_2.id {
                left.throws += 1;
                if round.hit {
                    left.hits += 1;
                }
            } else {
                right.throws += 1;
                if round.hit {
                    right.hits += 1;
                }
            }
            for additional in &round.additionals {
                match &additional.kind {
                    STRAFSCHLUCK => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            left.schluck += 1;
                        } else {
                            right.schluck += 1;
                        }
                    }
                    STRAFBIER => {
                        if additional.source.id == game.left_1.id || additional.source.id == game.left_2.id {
                            left.beer += 1;
                        } else {
                            right.beer += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("Seite  | Siege | Punkte | Trefferwahrscheinlichkeit | StrafS | StrafB");
    println!("Links  | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", left.wins, left.points, left.hits as f32 / left.throws as f32 * 100.0, left.hits, left.throws, left.schluck, left.beer);
    println!("Rechts | {:<5} | {:<5}  | {:.2} = {:>5} von {:<5}   | {:<5}  | {:<5}", right.wins, right.points, right.hits as f32 / right.throws as f32 * 100.0, right.hits, right.throws, right.schluck, right.beer);
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

fn player_name_from_id(player_id: u32, players: &Vec<TeamMember>) -> &str {
    for player in players {
        if player.id == player_id {
            return player.name;
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