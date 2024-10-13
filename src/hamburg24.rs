use std::i8::MIN;
use crate::{create_normal_rounds_left_right};
use crate::data::{ARC, bool_vec_from_int, Game, results_from_additionals, Team, TeamMember};
use crate::team_player_data::*;

pub fn create_spassturnier_24() ->Vec<Game>{
    let mut ret_vec = Vec::new();
    ret_vec.push(game_1(MINDESTGRÖSSE, HANNES, JONAS, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_2(BIERATENPARTEI, SEBI, BEEF, LURCHIE, LUISE, MICHI));
    ret_vec.push(game_3(CHROME, JEROME, CHRIS, NAME_KOMMT_SPÄTER, SARA, KATHI));
    ret_vec.push(game_4(NAME_KOMMT_SPÄTER, KATHI, SARA, BIERATENPARTEI, BEEF, SEBI));
    ret_vec.push(game_5(LURCHIE, MICHI, LUISE, MINDESTGRÖSSE, JONAS, HANNES));
    ret_vec.push(game_6(TOBÖNKE, SÖNKE, TOBIAS, CHROME, JEROME, CHRIS));
    ret_vec.push(game_7(LURCHIE, MICHI, LUISE, NAME_KOMMT_SPÄTER, SARA, KATHI));
    ret_vec.push(game_8(BIERATENPARTEI, SEBI, BEEF, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_9(MINDESTGRÖSSE, JONAS, HANNES, CHROME, JEROME, CHRIS));
    ret_vec.push(game_10(CHROME, JEROME, CHRIS, BIERATENPARTEI, BEEF, SEBI));
    ret_vec.push(game_11(TOBÖNKE, SÖNKE, TOBIAS, LURCHIE, LUISE, MICHI));
    ret_vec.push(game_12(NAME_KOMMT_SPÄTER, SARA, KATHI, MINDESTGRÖSSE, JONAS, HANNES));
    ret_vec.push(game_13(LURCHIE, MICHI, LUISE, CHROME, JEROME, CHRIS));
    ret_vec.push(game_14(MINDESTGRÖSSE, HANNES, JONAS, BIERATENPARTEI, SEBI, BEEF));
    ret_vec.push(game_15(NAME_KOMMT_SPÄTER, SARA, KATHI, TOBÖNKE, TOBIAS, SÖNKE));
    return ret_vec;
}

pub fn game_15(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1, 2), ARC::finish(&right_1, 2), ARC::finish(&right_2, 3)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(true, vec![4]), additionals, left_began);
    Game { result, match_number: 15, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_14(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1, 1), ARC::finish(&right_2, 5), ARC::finish(&right_1, 7)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,2,1,4]), additionals, left_began);
    Game { result, match_number: 14, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_13(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1, 2), ARC::finish(&right_1, 5), ARC::finish(&right_2, 5)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(true, vec![3,2,1]), additionals, left_began);
    Game { result, match_number: 13, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_12(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 7),ARC::beer(&right_1, 16), ARC::finish(&left_2, 17), ARC::finish(&left_1, 17)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,1,5,1,2,1,4,3]), additionals, left_began);
    Game { result, match_number: 12, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_11(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_2, 1),ARC::schluck(&right_2, 2), ARC::finish(&left_2, 5), ARC::finish(&right_2, 6), ARC::finish(&right_1, 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![2,5]), additionals, left_began);
    Game { result, match_number: 11, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_10(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_2, 1), ARC::finish(&left_1, 6), ARC::schluck(&left_2,6), ARC::finish(&right_1, 6), ARC::schluck(&right_2, 6), ARC::finish(&left_2, 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,1,1,2,1,1]), additionals, left_began);
    Game { result, match_number: 10, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_9(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 1), ARC::finish(&right_1, 2), ARC::finish(&right_2, 10)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(true, vec![3,7,1]), additionals, left_began);
    Game { result, match_number: 9, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_8(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 3), ARC::finish(&right_1, 6), ARC::finish(&left_2, 11), ARC::finish(&left_1, 13)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![3,1,1,2,4,3]), additionals, left_began);
    Game { result, match_number: 8, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_7(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1, 4), ARC::finish(&right_1, 5), ARC::finish(&right_2, 11)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,1,1,3,5,1]), additionals, left_began);
    Game { result, match_number: 7, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_6(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(&left_2, 9), ARC::finish(&right_2, 10), ARC::finish(&right_1, 10)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,1,3,2,1,3]), additionals, left_began);
    Game { result, match_number: 6, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_5(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 10), ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![1,2,1,3,3,1]), additionals, left_began);
    Game { result, match_number: 5, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_4(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_2, 5), ARC::finish(&right_1, 6), ARC::finish(&right_2, 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![2,2,1,2,1,1]), additionals, left_began);
    Game { result, match_number: 4, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_3(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_1, 1),ARC::finish(&left_1, 4), ARC::finish(&left_2, 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(true, vec![2,2,3]), additionals, left_began);
    Game { result, match_number: 3, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}
pub fn game_2(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 8), ARC::finish(&left_2, 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,bool_vec_from_int(false, vec![2,1,1,1,3,1]), additionals, left_began);
    Game { result, match_number: 2, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_1(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 6), ARC::finish(&right_2, 12)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,vec![false, true, false, false, true, false, true, false, true, false, false, true,true], additionals, left_began);
    Game { result, match_number: 1, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

