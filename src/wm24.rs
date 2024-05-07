use crate::{create_normal_rounds_left_right, first_game_new, game_3, game_4, second_game_new};
use crate::data::{ARC, bool_vec_from_int, Game, results_from_additionals, Team, TeamMember};
use crate::team_player_data::*;

pub fn create_all_games_wm_2024() -> Vec<Game>{
    let mut ret_vec : Vec<Game> = Vec::new();
    ret_vec.push(first_game_new(strammsein.clone(), sascha.clone(), jonas.clone(), white_claw.clone(), tobias.clone(), luise.clone()));
    ret_vec.push(second_game_new(gewertet.clone(), laura.clone(), hannes.clone(), dos_bros.clone(), flo.clone(), sebi.clone()));
    ret_vec.push(game_3(da_ham_sie.clone(), jerome.clone(), beef.clone(), wedelmedel.clone(), chris.clone(), malte.clone()));
    ret_vec.push(game_4(dos_bros.clone(), flo.clone(), sebi.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone()));
    ret_vec.push(game_5(wedelmedel.clone(), malte.clone(), chris.clone(), strammsein.clone(), jonas.clone(), sascha.clone()));
    ret_vec.push(game_6(gewertet.clone(), laura.clone(), hannes.clone(), white_claw.clone(), luise.clone(), tobias.clone()));
    ret_vec.push( game_7(wedelmedel.clone(), chris.clone(), malte.clone(), dos_bros.clone(), flo.clone(), sebi.clone()));
    ret_vec.push(game_8(da_ham_sie.clone(), jerome.clone(), beef.clone(), white_claw.clone(), luise.clone(), tobias.clone()));
    ret_vec.push(game_9(strammsein.clone(), jonas.clone(), sascha.clone(), gewertet.clone(), laura.clone(), hannes.clone()));
    ret_vec.push(game_10(gewertet.clone(), hannes.clone(), laura.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone()));
    ret_vec.push(game_11(white_claw.clone(), tobias.clone(), luise.clone(), wedelmedel.clone(), chris.clone(), malte.clone()));
    ret_vec.push(game_12(dos_bros.clone(), flo.clone(), sebi.clone(), strammsein.clone(), sascha.clone(), jonas.clone()));
    ret_vec.push(game_13(wedelmedel.clone(), chris.clone(), malte.clone(), gewertet.clone(), laura.clone(), hannes.clone()));
    ret_vec.push(game_14(strammsein.clone(), sascha.clone(), jonas.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone()));
    ret_vec.push(game_15(white_claw.clone(), tobias.clone(), luise.clone(), dos_bros.clone(), flo.clone(), sebi.clone()));
    ret_vec.push(game_16(white_claw.clone(), luise.clone(), tobias.clone(), strammsein.clone(), jonas.clone(), sascha.clone()));
    ret_vec.push(game_17(dos_bros.clone(), flo.clone(), sebi.clone(), gewertet.clone(), hannes.clone(), laura.clone()));
    ret_vec.push(game_18(wedelmedel.clone(), chris.clone(), malte.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone()));
    ret_vec.push(game_19(da_ham_sie.clone(), jerome.clone(), beef.clone(), dos_bros.clone(), flo.clone(), sebi.clone()));
    ret_vec.push(game_20(strammsein.clone(), sascha.clone(), jonas.clone(), wedelmedel.clone(), chris.clone(), malte.clone()));
    ret_vec.push(game_21(gewertet.clone(), laura.clone(), hannes.clone(), white_claw.clone(), luise.clone(), tobias.clone()));
    ret_vec.push(game_22(dos_bros.clone(), sebi.clone(), flo.clone(), wedelmedel.clone(), chris.clone(), malte.clone()));
    ret_vec.push(game_23(white_claw.clone(), tobias.clone(), luise.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone()));
    ret_vec.push(game_26(wedelmedel.clone(), chris.clone(), malte.clone(), white_claw.clone(), luise.clone(), tobias.clone()));
    ret_vec.push(game_27(strammsein.clone(), jonas.clone(), sascha.clone(), dos_bros.clone(), flo.clone(), sebi.clone()));
    ret_vec.push(game_29(da_ham_sie.clone(), jerome.clone(), beef.clone(), strammsein.clone(), sascha.clone(), jonas.clone()));
    ret_vec.push(game_30(dos_bros.clone(), flo.clone(), sebi.clone(), white_claw.clone(), tobias.clone(), luise.clone()));
    ret_vec.push(game_31(white_claw.clone(), tobias.clone(), luise.clone(), wedelmedel.clone(), chris.clone(), malte.clone()));
    ret_vec

}

pub fn game_31(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 7), ARC::finish(left_1.clone(), 16), ARC::finish(left_2.clone(), 16)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1,2,4, 1,4,1,3,1]), additionals, left_began);
    Game {result, match_number : 31, left_team, left_1, left_2, right_team, right_1, right_2, rounds}
}
pub fn game_30(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_1.clone(), 4), ARC::finish(left_1.clone(), 6), ARC::schluck(left_1.clone(),13), ARC::finish(right_1.clone(),13), ARC::finish(left_2.clone(), 16)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![4,3,9,1]), additionals, left_began);
    Game {result, match_number : 30, left_team, left_1, left_2, right_team, right_1, right_2, rounds}
}
pub fn game_29(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_2.clone(), 2), ARC::finish(left_1.clone(), 2), ARC::finish(left_2.clone(), 5)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 1, 2, 2]), additionals, left_began);
    Game { result, match_number: 29, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_27(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::beer(left_2.clone(), 3), ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![4, 2, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 27, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_26(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_2.clone(), 0), ARC::finish(left_1.clone(), 8), ARC::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 2, 1, 3, 1]), additionals, left_began);
    Game { result, match_number: 26, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_23(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_1.clone(), 0), ARC::schluck(left_2.clone(), 0), ARC::finish(right_1.clone(), 0), ARC::finish(right_2.clone(), 2)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1,1,1]), additionals, left_began);
    Game {result, match_number : 23, left_team, left_1, left_2, right_team, right_1, right_2, rounds}
}
pub fn game_22(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_2.clone(), 4), ARC::finish(left_1.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 1, 1, 3]), additionals, left_began);
    Game { result, match_number: 22, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_21(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::finish(left_2.clone(), 6), ARC::finish(left_1.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 3, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 21, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_20(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_2.clone(), 10), ARC::finish(left_1.clone(), 11), ARC::finish(left_2.clone(), 11)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 2, 2, 1, 2, 3]), additionals, left_began);
    Game { result, match_number: 20, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_19(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::finish(left_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 1, 2, 1, 1]), additionals, left_began);
    Game { result, match_number: 19, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_18(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 5), ARC::schluck(right_2.clone(), 5), ARC::schluck(left_2.clone(), 5), ARC::finish(right_2.clone(), 5)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 2, 2, 1]), additionals, left_began);
    Game { result, match_number: 18, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_17(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::schluck(right_1.clone(), 2), ARC::schluck(right_2.clone(), 2), ARC::finish(right_1.clone(), 10),
                           ARC::schluck(right_2.clone(), 10), ARC::finish(left_1.clone(), 10), ARC::finish(left_2.clone(), 13)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 1, 1, 1, 1, 5, 1, 2, 1]), additionals, left_began);
    Game { result, match_number: 17, left_team, left_1: left_2, left_2: left_1, right_team, right_1, right_2, rounds }
}

pub fn game_16(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(right_1.clone(), 1), ARC::finish(left_2.clone(), 1), ARC::finish(left_1.clone(), 4)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![3, 1, 1]), additionals, left_began);
    Game { result, match_number: 16, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_15(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 9), ARC::finish(left_1.clone(), 12), ARC::finish(right_2.clone(), 13)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 6, 3, 1, 3]), additionals, left_began);
    Game { result, match_number: 15, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_14(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![3, 3, 1]), additionals, left_began);
    Game { result, match_number: 14, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_13(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 8), ARC::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 2, 1, 3, 1]), additionals, left_began);
    Game { result, match_number: 13, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_12(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 8), ARC::finish(left_1.clone(), 9), ARC::finish(right_2.clone(), 12)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 6, 2, 1, 2]), additionals, left_began);
    Game { result, match_number: 12, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_11(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::schluck(left_2.clone(), 4), ARC::finish(left_1.clone(), 10), ARC::schluck(left_2.clone(), 12), ARC::finish(right_1.clone(), 12), ARC::finish(right_2.clone(), 12)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![1, 1, 3, 1, 2, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 11, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_10(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(right_1.clone(), 5), ARC::finish(right_2.clone(), 7)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![2, 3, 3]), additionals, left_began);
    Game { result, match_number: 10, left_team, left_1: left_2, left_2: left_1, right_team, right_1, right_2, rounds }
}

pub fn game_9(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::finish(left_2.clone(), 4)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 1, 3]), additionals, left_began);
    Game { result, match_number: 9, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_8(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = true;
    let additionals = vec![ARC::finish(left_1.clone(), 4), ARC::beer(left_2.clone(), 4), ARC::beer(right_2.clone(), 5), ARC::finish(left_2.clone(), 14)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(true, vec![1, 2, 3, 2, 1, 3, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 8, left_team, left_1, left_2, right_team, right_1, right_2, rounds }
}

pub fn game_7(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
    let left_began = false;
    let additionals = vec![ARC::finish(right_1.clone(), 6), ARC::finish(right_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began, &left_team);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), bool_vec_from_int(false, vec![2, 1, 1, 1, 1, 1, 1, 1]), additionals, left_began);
    Game { result, match_number: 7, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

pub fn game_6(left_team: crate::Team, left_1: crate::TeamMember, left_2: crate::TeamMember, right_team: crate::Team, right_1: crate::TeamMember, right_2: crate::TeamMember) -> crate::Game {
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