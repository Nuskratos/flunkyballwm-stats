use crate::calc::calculation::average;
use crate::calc::drink_avg_data::DrinkAvgStats;
use crate::calc::drink_finished_data::DrinkFinishedStats;
use crate::data::{Game, Team, TeamMember};
use crate::data::AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};
use crate::util::{player_is_in_game, team_id_from_player};

pub fn calculate_avg(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, finished_stats: &DrinkFinishedStats, schluck_effect: f32) -> DrinkAvgStats {
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

pub fn calculate_finished(games: &Vec<Game>, player: &TeamMember, teams: &Vec<Team>, schluck_effect: f32) -> DrinkFinishedStats {
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
