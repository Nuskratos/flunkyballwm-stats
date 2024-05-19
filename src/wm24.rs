use crate::{create_normal_rounds_left_right};
use crate::data::{Additional, ARC, bool_vec_from_int, create_normalized_rounds, Game, results_from_additionals, Team, TeamMember};
use crate::data::AdditionalType::{FINISHED, STRAFSCHLUCK};
use crate::team_player_data::*;

pub fn create_all_games_without_dos_or_wedelmedel() -> Vec<Game>{
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(first_game_new(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone()));
    ret_vec.push(game_6(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_8(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_9(STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone(), GEWERTET.clone(), LAURA.clone(), HANNES.clone()));
    ret_vec.push(game_10(GEWERTET.clone(), HANNES.clone(), LAURA.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_14(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_16(WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone(), STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone()));
    ret_vec.push(game_21(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_23(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_29(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone()));
    ret_vec
}


pub fn create_games_without_me_or_sebi() -> Vec<Game>{
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(first_game_new(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone()));
    ret_vec.push(game_5(WEDELMEDEL.clone(), MALTE.clone(), CHRIS.clone(), STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone()));
    ret_vec.push(game_6(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_9(STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone(), GEWERTET.clone(), LAURA.clone(), HANNES.clone()));
    ret_vec.push(game_11(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_13(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), GEWERTET.clone(), LAURA.clone(), HANNES.clone()));
    ret_vec.push(game_16(WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone(), STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone()));
    ret_vec.push(game_20(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_21(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_26(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_31(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec

}

pub fn create_all_games_wm_2024() -> Vec<Game> {
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(first_game_new(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone()));
    ret_vec.push(game_2(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_3(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_4(DOS_BROS.clone(), SEBI.clone(), FLO.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_5(WEDELMEDEL.clone(), MALTE.clone(), CHRIS.clone(), STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone()));
    ret_vec.push(game_6(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_7(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_8(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_9(STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone(), GEWERTET.clone(), LAURA.clone(), HANNES.clone()));
    ret_vec.push(game_10(GEWERTET.clone(), HANNES.clone(), LAURA.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_11(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_12(DOS_BROS.clone(), FLO.clone(), SEBI.clone(), STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone()));
    ret_vec.push(game_13(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), GEWERTET.clone(), LAURA.clone(), HANNES.clone()));
    ret_vec.push(game_14(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_15(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_16(WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone(), STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone()));
    ret_vec.push(game_17(DOS_BROS.clone(), FLO.clone(), SEBI.clone(), GEWERTET.clone(), HANNES.clone(), LAURA.clone()));
    ret_vec.push(game_18(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_19(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_20(STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_21(GEWERTET.clone(), LAURA.clone(), HANNES.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_22(DOS_BROS.clone(), SEBI.clone(), FLO.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_23(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_26(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), WHITE_CLAW.clone(), LUISE.clone(), TOBIAS.clone()));
    ret_vec.push(game_27(STRAMMSEIN.clone(), JONAS.clone(), SASCHA.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_29(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), STRAMMSEIN.clone(), SASCHA.clone(), JONAS.clone()));
    ret_vec.push(game_30(DOS_BROS.clone(), FLO.clone(), SEBI.clone(), WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone()));
    ret_vec.push(game_31(WHITE_CLAW.clone(), TOBIAS.clone(), LUISE.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec
}

pub fn game_31(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 7), ARC::finish(left_1.clone(), 16), ARC::finish(left_2.clone(), 16)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 2, 4, 1, 4, 1, 3, 1]), additionals, left_began);
    Game { result, match_number: 31, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_30(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_1.clone(), 4), ARC::finish(left_1.clone(), 6), ARC::schluck(left_1.clone(), 13), ARC::finish(right_1.clone(), 13), ARC::finish(left_2.clone(), 16)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![4, 3, 9, 1]), additionals, left_began);
    Game { result, match_number: 30, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_29(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_2.clone(), 2), ARC::finish(left_1.clone(), 2), ARC::finish(left_2.clone(), 5)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 1, 2, 2]), additionals, left_began);
    Game { result, match_number: 29, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_27(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::beer(left_2.clone(), 3), ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![4, 2, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 27, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_26(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_2.clone(), 0), ARC::finish(left_1.clone(), 8), ARC::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 2, 1, 3, 1]), additionals, left_began);
    Game { result, match_number: 26, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_23(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_1.clone(), 0), ARC::schluck(left_2.clone(), 0), ARC::finish(right_1.clone(), 0), ARC::finish(right_2.clone(), 2)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 1, 1]), additionals, left_began);
    Game { result, match_number: 23, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_22(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game { //TODO This needs a manual change for the running, because chris ran twice
    let left_began = true;
    let additionals = vec![ARC::finish(left_2.clone(), 4), ARC::finish(left_1.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 1, 1, 3]), additionals, left_began);
    Game { result, match_number: 22, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_21(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_2.clone(), 6), ARC::finish(right_1.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 3, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 21, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_20(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_2.clone(), 10), ARC::finish(left_1.clone(), 11), ARC::finish(left_2.clone(), 11)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 2, 2, 1, 2, 3]), additionals, left_began);
    Game { result, match_number: 20, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_19(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::finish(left_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 1, 2, 1, 1]), additionals, left_began);
    Game { result, match_number: 19, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_18(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 5), ARC::schluck(right_2.clone(), 5), ARC::schluck(left_2.clone(), 5), ARC::finish(right_2.clone(), 5)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 2, 2, 1]), additionals, left_began);
    Game { result, match_number: 18, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_17(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_1.clone(), 2), ARC::schluck(right_2.clone(), 2), ARC::finish(right_1.clone(), 10),
                           ARC::schluck(right_2.clone(), 10), ARC::finish(left_1.clone(), 10), ARC::finish(left_2.clone(), 13)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 1, 1, 1, 1, 5, 1, 2, 1]), additionals, left_began);
    Game { result, match_number: 17, left_team, left_1: left_2, left_2: left_1, right_team, right_1, right_2, rounds }
}

pub fn game_16(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(right_1.clone(), 1), ARC::finish(left_2.clone(), 1), ARC::finish(left_1.clone(), 4)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![3, 1, 1]), additionals, left_began);
    Game { result, match_number: 16, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_15(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 9), ARC::finish(left_1.clone(), 12), ARC::finish(right_2.clone(), 13)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 6, 3, 1, 3]), additionals, left_began);
    Game { result, match_number: 15, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_14(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![3, 3, 1]), additionals, left_began);
    Game { result, match_number: 14, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_13(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 8), ARC::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 2, 1, 3, 1]), additionals, left_began);
    Game { result, match_number: 13, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_12(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 8), ARC::finish(left_1.clone(), 9), ARC::finish(right_2.clone(), 12)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 6, 2, 1, 2]), additionals, left_began);
    Game { result, match_number: 12, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_11(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_2.clone(), 4), ARC::finish(left_1.clone(), 10), ARC::schluck(left_2.clone(), 12), ARC::finish(right_1.clone(), 12), ARC::finish(right_2.clone(), 12)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 1, 3, 1, 2, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 11, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_10(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 5), ARC::finish(right_2.clone(), 7)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 3, 3]), additionals, left_began);
    Game { result, match_number: 10, left_team, left_1: left_2, left_2: left_1, right_team, right_1, right_2, rounds }
}

pub fn game_9(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::finish(left_2.clone(), 4)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 1, 3]), additionals, left_began);
    Game { result, match_number: 9, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_8(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::beer(left_2.clone(), 4), ARC::beer(right_2.clone(), 5), ARC::finish(left_2.clone(), 14)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 2, 3, 2, 1, 3, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 8, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_7(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 1, 1, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 7, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

pub fn game_6(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_2.clone(), 13), ARC::beer(right_1.clone(), 15), ARC::schluck(right_1.clone(), 17), ARC::finish(left_1.clone(), 28), ARC::finish(left_2.clone(), 28)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![7, 1, 5, 1, 1, 1, 1, 3, 6, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

pub fn game_5(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_1.clone(), 4), ARC::finish(right_1.clone(), 10), ARC::finish(right_2.clone(), 10)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, false, false, true, false, false, true, false, false, true, true], additionals, left_began);
    Game { result, match_number: 5, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

fn game_4(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_2.clone(), 6), ARC::finish(right_1.clone(), 7), ARC::finish(left_1.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_2.clone(), left_1.clone(), right_1.clone(), right_2.clone(), vec![false, false, true, true, true, false, true, true, true], additionals, left_began);
    Game { result, match_number: 4, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
fn game_2(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let mut additionals = vec![ARC::finish(right_1.clone(), 4), ARC::finish(right_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, true, true, true, true, false, true], additionals.clone(), left_began);
    Game { result, match_number: 2, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

fn first_game_new(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals: Vec<ARC> = vec![ARC::schluck(left_1.clone(), 0), ARC::finish(left_1.clone(), 14), ARC::finish(left_2.clone(), 14)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true], additionals.clone(), left_began);
    Game { result, match_number: 1, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}


fn game_3(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 6), ARC::finish(left_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, true, true, false, false, true, true], additionals, left_began);
    Game { result, match_number: 3, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}



fn first_game_old(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let mut rounds = create_normalized_rounds(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true]);
    let straf_sasch = Additional { kind: STRAFSCHLUCK, source: left_1.clone() };
    rounds.first_mut().unwrap().additionals(vec![straf_sasch]);

    let finished_sasch = Additional { kind: FINISHED, source: left_1.clone() };
    let finished_jonas = Additional { kind: FINISHED, source: left_2.clone() };
    rounds.last_mut().unwrap().additionals(vec![finished_sasch, finished_jonas]);
    let result = crate::data::Result { points_left: 14, points_right: 0 };
    Game { result, match_number: 1, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

//THIS IS FAULTY, because Florian was finished but according to this sheet ran in the 2nd to last throw
fn second_game_old_faulty(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let result = crate::data::Result { points_left: 0, points_right: 14 };
    let mut rounds = create_normalized_rounds(right_1.clone(), right_2.clone(), left_1.clone(), left_2.clone(), vec![true, true, true, true, true, false, true]); //
    let finish_flo = Additional { kind: FINISHED, source: right_1.clone() };
    let finish_sebi = Additional::finish(right_2.clone());
    rounds.get_mut(4).unwrap().additionals(vec![finish_flo]);
    rounds.last_mut().unwrap().additionals(vec![finish_sebi]);
    Game { result, left_1, left_2, right_1, right_2, match_number: 2, left_team, right_team, rounds }
}