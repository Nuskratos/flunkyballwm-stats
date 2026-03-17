use crate::{create_normal_rounds_left_right};
use crate::data::{ARC, bool_vec_from_int, Game, results_from_additionals, Team, TeamMember};
use crate::team_player_data::*;

pub fn print_specials_2025(){
    println!("Special things happened in WM 2025:");
    // TODO Links begann wie häufig?
    // TODO Stats für wer wie oft Schnucken gewonnen hat
    println!("{} NIX WICHTIGES", FLO.name);
}
pub fn create_all_games_wm_2025() -> Vec<Game> {
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(game_1(FLANKY_KRIEG.clone(), PATRICK.clone(), CHRISTOPH.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_2(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_3(RESERVISTEN.clone(), TOBIAS.clone(), JAN_W.clone(), PAEDAGOGISCH.clone(), LENA.clone(), LUISE.clone()));
    ret_vec.push(game_4(DOS_BROS.clone(), FLO.clone(), SEBI.clone(), RESERVISTEN.clone(), TOBIAS.clone(), JAN_W.clone()));
    ret_vec.push(game_5(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone()));
    ret_vec.push(game_6(MC_FLANKY.clone(), MARCEL.clone(), JACKY.clone(), FLANKY_KRIEG.clone(), PATRICK.clone(), CHRISTOPH.clone()));
    ret_vec.push(game_7(WEDELMEDEL.clone(), CHRIS.clone(), MALTE.clone(), MC_FLANKY.clone(), MARCEL.clone(), JACKY.clone()));
    ret_vec.push(game_8(RESERVISTEN.clone(), JAN_W.clone(), TOBIAS.clone(), DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone()));
    ret_vec.push(game_9(PAEDAGOGISCH.clone(), LUISE.clone(), LENA.clone(), DOS_BROS.clone(), FLO.clone(), SEBI.clone()));
    ret_vec.push(game_10(DA_HAM_SIE.clone(), JEROME.clone(), BEEF.clone(), PAEDAGOGISCH.clone(), LUISE.clone(), LENA.clone()));
    ret_vec.push(game_11(MC_FLANKY, MARCEL, JACKY, RESERVISTEN, TOBIAS, JAN_W));
    ret_vec.push(game_12(FLANKY_KRIEG, CHRISTOPH, PATRICK, WEDELMEDEL, CHRIS, MALTE));
    ret_vec.push(game_13(RESERVISTEN, JAN_W, TOBIAS, FLANKY_KRIEG, PATRICK, CHRISTOPH));
    ret_vec.push(game_14(PAEDAGOGISCH, LENA, LUISE, MC_FLANKY, JACKY, MARCEL));
    ret_vec.push(game_15(DOS_BROS, SEBI, FLO, DA_HAM_SIE, JEROME, BEEF));
    ret_vec.push(game_16(MC_FLANKY, JACKY, MARCEL, DOS_BROS, FLO, SEBI));
    ret_vec.push(game_17(PAEDAGOGISCH, LUISE, LENA, FLANKY_KRIEG, CHRISTOPH, PATRICK));
    ret_vec.push(game_18(WEDELMEDEL, CHRIS, MALTE, RESERVISTEN, JAN_W, TOBIAS));
    ret_vec.push(game_19(PAEDAGOGISCH, LUISE, LENA, WEDELMEDEL, CHRIS, MALTE));
    ret_vec.push(game_20(DOS_BROS, FLO, SEBI, FLANKY_KRIEG, PATRICK, CHRISTOPH));
    ret_vec.push(game_21(DA_HAM_SIE, JEROME, BEEF, MC_FLANKY, JACKY, MARCEL));
    //ret_vec.push(game_pl4(DOS_BROS, FLO, SEBI, FLANKY_KRIEG, PATRICK, CHRISTOPH));
    ret_vec
}

pub fn game_template(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 6), ARC::finish(&right_2, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![]), additionals, left_began);
    Game { result, match_number: 7, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_pl4(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = true;
    let additionals = vec![ARC::finish(&left_2, 4), ARC::schluck(&right_2, 9),ARC::finish(&left_1, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![5,1,1,3]), additionals, left_began);
    Game { result, match_number: 22, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

pub fn game_21(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 2), ARC::finish(&left_2, 4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![5]), additionals, left_began);
    Game { result, match_number: 21, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_20(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 8), ARC::finish(&right_2, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,2,2,1]), additionals, left_began);
    Game { result, match_number: 20, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_19(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 4), ARC::finish(&right_2, 4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,3]), additionals, left_began);
    Game { result, match_number: 19, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_18(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 4), ARC::finish(&left_2, 4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,3]), additionals, left_began);
    Game { result, match_number: 18, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_17(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 10), ARC::finish(&right_2, 12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,5,2,2,1,1,1]), additionals, left_began);
    Game { result, match_number: 17, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_16(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 3),ARC::finish(&right_1, 3), ARC::finish(&right_2, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,2,1,3,2]), additionals, left_began);
    Game { result, match_number: 16, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_15(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_2, 3), ARC::beer(&left_2, 3),ARC::finish(&right_1, 3), ARC::finish(&right_2, 3)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![4]), additionals, left_began);
    Game { result, match_number: 15, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_14(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&right_2, 6), ARC::finish(&right_1, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,2,2,1,1,1,1]), additionals, left_began);
    Game { result, match_number: 14, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_13(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 5), ARC::finish(&left_2, 5)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,3]), additionals, left_began);
    Game { result, match_number: 13, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_12(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 1),ARC::finish(&right_1, 6), ARC::finish(&right_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,2,1,1,2,2,1]), additionals, left_began);
    Game { result, match_number: 12, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_11(left_team_b: Team, left_1p:TeamMember, left_2p: TeamMember, right_teamp: Team, right_1p: TeamMember, right_2p:TeamMember)-> Game{
    let left_team: Team = left_team_b.clone();
    let left_1: TeamMember = left_1p.clone();
    let left_2: TeamMember = left_2p.clone();
    let right_team: Team = right_teamp.clone();
    let right_1: TeamMember = right_1p.clone();
    let right_2: TeamMember = right_2p.clone();
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 5),ARC::finish(&right_1, 6), ARC::finish(&right_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,1,2,3,1]), additionals, left_began);
    Game { result, match_number: 11, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_10(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 6),ARC::finish(&left_1, 9), ARC::finish(&left_2, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![5,2,2,1]), additionals, left_began);
    Game { result, match_number: 10, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_9(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_2, 2),ARC::finish(&right_1, 2), ARC::finish(&right_2, 5)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,3,1,1]), additionals, left_began);
    Game { result, match_number: 9, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_8(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_1, 5), ARC::finish(&left_2, 5),ARC::finish(&right_1, 5),ARC::finish(&left_1, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,2,3]), additionals, left_began);
    Game { result, match_number: 8, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_7(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_2,3), ARC::finish(&right_1, 5),ARC::finish(&left_1, 6), ARC::finish(&left_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,6]), additionals, left_began);
    Game { result, match_number: 7, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 12), ARC::finish(&right_2, 12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,2,3,2,1,1]), additionals, left_began);
    Game { result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 5),ARC::finish(&left_1, 5), ARC::finish(&right_1, 6), ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,2,3]), additionals, left_began);
    Game { result, match_number: 5, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}

pub fn game_4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 4), ARC::finish(&left_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,3,1,1]), additionals, left_began);
    Game { result, match_number: 4, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 6), ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,2,1,2,2]), additionals, left_began);
    Game { result, match_number: 3, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_2, 5), ARC::finish(&left_1, 7)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,5]), additionals, left_began);
    Game { result, match_number: 2, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
pub fn game_1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 6), ARC::schluck(&right_2,6),ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, vec![true, false, true, false, false, true, true], additionals, left_began);
    Game { result, match_number: 1, left_team, left_1, left_2, right_1, right_2, right_team, rounds }
}
