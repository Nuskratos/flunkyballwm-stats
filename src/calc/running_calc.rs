use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::current_exe;
use crate::calc::drink_calc::calculate_drinking_speed_without_team;
use crate::calc::running_data::{RunningDiff, TeamRunningStatistics};
use crate::data::{Game, Team, TeamMember};
use crate::util::{player_is_in_game, team_from_player, team_id_from_player, team_is_in_game};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::team_player_data::WHITE_CLAW;


pub fn calculate_running_speeds(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32) -> TeamRunningStatistics {
    let mut team_diff_map: HashMap<u32, RunningDiff> = HashMap::new();
    for team in teams {
        for game in games {
            if !team_is_in_game(game, team) {
                continue;
            }
            let mut straf_beer_map : HashMap<u32, f32> = HashMap::new();
            let mut current_diff = RunningDiff::create();
            let mut current_run = 0.0;
            // Calculating speeds of enemy team
            let enemy_team = if &game.left_team == team { &game.right_team } else { &game.left_team };
            let enemy_vec = vec![enemy_team.clone()];
            let other_speeds = calculate_drinking_speed_without_team(games, players, &enemy_vec, schluck_effect, team);
            let team_1_drink_speed = other_speeds.speeds.iter().find(|x| x.player_name == enemy_team.member_1.name).unwrap().drink_avg.all_speed();
            let team_2_drink_speed = other_speeds.speeds.iter().find(|x| x.player_name == enemy_team.member_2.name).unwrap().drink_avg.all_speed();
            let mut team1_finished = false;
            let mut team2_finished = false;
            for round in &game.rounds {
                if team_id_from_player(round.runner.id, teams) == team.id && round.hit {
                    current_run += current_diff.baseline;
                }
                for add in &round.additionals {
                    let this_run = running_amount_for_this_beer(&straf_beer_map, add.source.id, current_run);
                    match &add.kind {
                        FINISHED => {
                            if add.source.id == enemy_team.member_1.id {
                                current_diff.diff_to_expected += team_1_drink_speed - this_run;
                                current_diff.run_amount += this_run;
                                team1_finished = true;
                            }
                            if add.source.id == enemy_team.member_2.id {
                                current_diff.diff_to_expected += team_2_drink_speed - this_run;
                                current_diff.run_amount += this_run;
                                team2_finished = true;
                            }
                        }
                        STRAFSCHLUCK => {
                            if team_from_player(add.source.id, teams) == team {
                                current_run += schluck_effect;
                            }
                        }
                        STRAFBIER => {
                            if add.source.id == enemy_team.member_1.id {
                                current_diff.diff_to_expected += team_1_drink_speed - this_run - 1.0;
                                current_diff.run_amount += this_run;
                            }
                            if add.source.id == enemy_team.member_2.id {
                                current_diff.diff_to_expected += team_2_drink_speed - this_run - 1.0;
                                current_diff.run_amount += this_run;
                            }
                            straf_beer_map.insert(add.source.id, current_run);
                        }
                    }
                }
            }
            if !team1_finished && team_1_drink_speed.floor() <= current_run { // acting if they had finished the next round
                current_diff.diff_to_expected += team_1_drink_speed - current_run - 1.0;
                current_diff.run_amount += current_run + current_diff.baseline;
            }
            if !team2_finished && team_2_drink_speed.floor() <= current_run {
                current_diff.diff_to_expected += team_2_drink_speed - current_run - 1.0;
                current_diff.run_amount += current_run + current_diff.baseline;
            }
            team_diff_map.entry(team.id).and_modify(|x| x.add(&current_diff)).or_insert(current_diff);
        }
    }
    let mut ret_vec: Vec<(Team, RunningDiff)> = Vec::new();
    for team in team_diff_map {
        ret_vec.push((teams.iter().find(|x| x.id == team.0).unwrap().clone(), team.1));
    }
    ret_vec.sort_by(|a, b| a.1.round_length().partial_cmp(&b.1.round_length()).unwrap_or(Ordering::Less));
    TeamRunningStatistics { speeds: ret_vec, schluck_effect }
}

fn running_amount_for_this_beer(map : & HashMap<u32, f32>, id: u32, hit_amount: f32) -> f32{
    if map.get(&id).is_some(){
        return hit_amount - map.get(&id).unwrap();
    }
    hit_amount
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;
    use crate::calc::running_calc::calculate_running_speeds;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3, TEST_TEAM4};
    use crate::util::{players_from_games, teams_from_games};
    use crate::util::test::{game_1st_finish_2straf, game_2nd_finish, game_3rd_finish};

    #[test]
    fn team_in_2_took_3() {
        let games = vec![game_2nd_finish(TEST_TEAM2, TEST_TEAM3), game_3rd_finish(TEST_TEAM2, TEST_TEAM1)];
        let teams = teams_from_games(&games);
        let players = players_from_games(&games);

        let data = calculate_running_speeds(&games, &players, &teams, 0.8);
        data.print();
        let speed_team_1 = data.speeds.iter().find(|x| x.0.id == TEST_TEAM1.id).unwrap().1.round_length();
        let speed_team_3 = data.speeds.iter().find(|x| x.0.id == TEST_TEAM3.id).unwrap().1.round_length();
        assert!(approx_eq!(f32, speed_team_1, 2.0/3.0));
        assert!(approx_eq!(f32, speed_team_3, 1.5));
    }

    // team in 2 wasnt finished in 3
    #[test]
    fn team_in_2_wasnt_done_in_2() {
        let games = vec![game_2nd_finish(TEST_TEAM2, TEST_TEAM3), game_3rd_finish(TEST_TEAM1, TEST_TEAM2),game_2nd_finish(TEST_TEAM2, TEST_TEAM4)];
        let teams = teams_from_games(&games);
        let players = players_from_games(&games);

        let data = calculate_running_speeds(&games, &players, &teams, 0.8);
        data.print();
        let speed_team_1 = data.speeds.iter().find(|x| x.0.id == TEST_TEAM1.id).unwrap().1.round_length();
        let speed_team_3 = data.speeds.iter().find(|x| x.0.id == TEST_TEAM3.id).unwrap().1.round_length();
        assert!(approx_eq!(f32, speed_team_1, 2.0/3.0));
        assert!(approx_eq!(f32, speed_team_3, 1.25));
    }

    #[test]
    fn team_in_2_was_done_after_2_with_schluck() {
        let games = vec![game_2nd_finish(TEST_TEAM2, TEST_TEAM3), game_1st_finish_2straf(TEST_TEAM2, TEST_TEAM1)];
        let teams = teams_from_games(&games);
        let players = players_from_games(&games);

        let data = calculate_running_speeds(&games, &players, &teams, 0.8);
        data.print();
        let speed_team_1 = data.speeds.iter().find(|x| x.0.id == TEST_TEAM1.id).unwrap().1.round_length();
        assert!(approx_eq!(f32, speed_team_1, 1.0- 0.6/2.6));
    }

    // TODO Test this with simulated data to see if any metric is close to the +- 0
}