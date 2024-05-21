use crate::calc::calculation::average;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;
use crate::calc::drink_total_data::{DrinkingSpeedVec, PlayerDrinkingSpeed};
use crate::data::{Game, Team, TeamMember};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::util::{player_is_in_game, team_id_from_player, team_is_in_game};

pub fn calculate_drinking_speed_without_team(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32, team_to_be_removed: &Team) -> DrinkingSpeedVec {
    let mut playerspeeds: Vec<PlayerDrinkingSpeed> = Vec::new();
    for player in players {
        let mut filtered_games: Vec<Game> = games.clone();
        filtered_games.retain(|x| !team_is_in_game(x, team_to_be_removed));
        let finished = calculate_finished(&filtered_games, player, teams, schluck_effect);
        let averages = calculate_avg(&filtered_games, player, teams, &finished, schluck_effect);
        playerspeeds.push(PlayerDrinkingSpeed { drink_finished: finished, drink_avg: averages, player_name: String::from(player.name) });
    }
    playerspeeds.sort_by(|a, b| a.custom_cmp(&b).unwrap());
    DrinkingSpeedVec { schluck_effect, speeds: playerspeeds }
}

pub fn calculate_drinking_speed(games: &Vec<Game>, players: &Vec<TeamMember>, teams: &Vec<Team>, schluck_effect: f32) -> DrinkingSpeedVec {
    let mut playerspeeds: Vec<PlayerDrinkingSpeed> = Vec::new();
    for player in players {
        let finished = calculate_finished(games, player, teams, schluck_effect);
        let averages = calculate_avg(games, player, teams, &finished, schluck_effect);
        playerspeeds.push(PlayerDrinkingSpeed { drink_finished: finished, drink_avg: averages, player_name: String::from(player.name) });
    }
    playerspeeds.sort_by(|a, b| a.custom_cmp(&b).unwrap());
    DrinkingSpeedVec { schluck_effect, speeds: playerspeeds }
}

// TODO these functions could be refactored into working with maps for a runtime improvement
fn calculate_avg(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, finished_stats: &DrinkFinishedStats, schluck_effect: f32) -> DrinkAvgStats {
    let mut avg_stats = DrinkAvgStats::new();
    for game in games {
        if !player_is_in_game(game, player) {
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

fn calculate_finished(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, schluck_effect: f32) -> DrinkFinishedStats {
    let mut finshed_stats = DrinkFinishedStats::new();
    for game in games {
        if !player_is_in_game(game, player) {
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

#[cfg(test)]
mod test{
    use float_cmp::approx_eq;
    use crate::calc::drink_calc::{calculate_drinking_speed, calculate_drinking_speed_without_team};
    use crate::team_player_data::{TEST_PLAYER1, TEST_PLAYER2, TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_2nd_finish, game_3rd_finish};

    #[test]
    fn culled_team_calculation_works(){
        let games = vec![game_2nd_finish(TEST_TEAM1, TEST_TEAM2),game_3rd_finish(TEST_TEAM1, TEST_TEAM2), game_3rd_finish(TEST_TEAM1, TEST_TEAM3)];
        let players = vec![TEST_TEAM1.member_1, TEST_TEAM1.member_2];
        let teams = vec![TEST_TEAM1, TEST_TEAM2, TEST_TEAM3];
        let culled_data = calculate_drinking_speed_without_team(&games, &players, &teams, 0.5, &TEST_TEAM3);
        let first_culled_speed = culled_data.speeds.iter().find(|x|x.player_name == TEST_PLAYER1.name).unwrap().drink_avg.all_speed();
        assert!(approx_eq!(f32, first_culled_speed, 2.5));

        let total_data = calculate_drinking_speed(&games, &players, &teams, 0.5);
        let first_total_speed = total_data.speeds.iter().find(|x|x.player_name == TEST_PLAYER1.name).unwrap().drink_avg.all_speed();
        assert!(approx_eq!(f32, first_total_speed,  8f32 / 3f32 ));
    }
}
