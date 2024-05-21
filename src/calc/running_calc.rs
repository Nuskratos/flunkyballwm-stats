use std::collections::HashMap;
use std::env::current_exe;
use crate::calc::drink_calc::calculate_drinking_speed_without_team;
use crate::calc::running_data::{RunningDiff, RunningStatistics};
use crate::data::{Game, Team, TeamMember};
use crate::util::{player_is_in_game, team_from_player, team_id_from_player, team_is_in_game};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};


pub fn calculate_running_speeds(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32) -> RunningStatistics {
    let mut team_diff_map: HashMap<u32, RunningDiff> = HashMap::new();
    for team in teams {
        for game in games {
            if !team_is_in_game(game, team) {
                continue;
            }
            let mut current_diff = RunningDiff::create();
            // Calculating speeds of enemy team
            let enemy_team = if &game.left_team == team { &game.right_team } else { &game.left_team };
            let enemy_vec = vec![enemy_team.clone()];
            let other_speeds = calculate_drinking_speed_without_team(games, players, &enemy_vec, schluck_effect, team);
            let team_1_drink_speed = other_speeds.speeds.iter().find(|x| x.player_name == enemy_team.member_1.name).unwrap().drink_avg.all_speed();
            let team_2_drink_speed = other_speeds.speeds.iter().find(|x| x.player_name == enemy_team.member_2.name).unwrap().drink_avg.all_speed();

            for round in &game.rounds {
                if team_id_from_player(round.runner.id, teams) == team.id && round.hit {
                    current_diff.run_amount += current_diff.baseline;
                }
                for add in &round.additionals {
                    match &add.kind {
                        FINISHED => {
                            if add.source.id == enemy_team.member_1.id {
                                current_diff.diff_to_expected += team_1_drink_speed - current_diff.run_amount;
                            }
                            if add.source.id == enemy_team.member_2.id {
                                current_diff.diff_to_expected += team_2_drink_speed - current_diff.run_amount;
                            }
                        }
                        STRAFSCHLUCK => {
                            if team_from_player(add.source.id, teams) == enemy_team {
                                current_diff.run_amount += schluck_effect;
                            }
                        }
                        STRAFBIER => {
                            if add.source.id == enemy_team.member_1.id {
                                current_diff.diff_to_expected += team_1_drink_speed - current_diff.run_amount - 1.0; // or should the 1 be baseline?
                            }
                            if add.source.id == enemy_team.member_2.id {
                                current_diff.diff_to_expected += team_2_drink_speed - current_diff.run_amount - 1.0;
                            }
                        }
                    }
                }
            }
            team_diff_map.entry(team.id).and_modify(|x| x.add(&current_diff)).or_insert(current_diff);
        }
    }
    let mut ret_vec: Vec<(u32, RunningDiff)> = Vec::new();
    for team in team_diff_map {
        ret_vec.push((team.0, team.1));
    }
    ret_vec.sort_by(|a,b| a.1.diff_to_baseline().partial_cmp(&b.1.diff_to_baseline()).unwrap());
    RunningStatistics{speeds: ret_vec, schluck_effect}
}

#[cfg(test)]
mod test{
    use crate::calc::running_calc::calculate_running_speeds;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::{players_from_games, teams_from_games};
    use crate::util::test::{game_2nd_finish, game_3rd_finish};

    fn team_in_2_took_3(){
        let games = vec![game_2nd_finish(TEST_TEAM2, TEST_TEAM3), game_3rd_finish(TEST_TEAM2, TEST_TEAM1)];
        let teams = teams_from_games(&games);
        let players = players_from_games(&games);

        let data = calculate_running_speeds(&games, &players, &teams, 0.8);
        data.print();
    }

    // team in 2 wasnt finished in 3

    // team in 2 was only finished after strafschluck
}