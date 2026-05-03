use crate::{create_normal_rounds_left_right};
use crate::data::{ARC, bool_vec_from_int, Game, results_from_additionals, Team, TeamMember};
use crate::team_player_data::*;
use crate::util::convert_first_throw_games;

pub fn print_specials_2025(){
    println!("Special things happened in WM 2025:");
    // TODO Links begann wie häufig?
    // TODO Stats für wer wie oft Schnucken gewonnen hat
    println!("{} NIX WICHTIGES", FLO.name());
}
pub fn create_all_games_wm_2026() -> Vec<Game> {
    let mut ret_vec: Vec<Game> = Vec::new();
    // Side A
    ret_vec.push(game_a1(HELLWACH, JAKOB_HELLWACH, NIKO, JGA2, KATI_JGA, SOPHIE));
    ret_vec.push(game_a2(COON_AND_FRIENDS, CHRIS, DAVID, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_a3(JESS_RON, ARON, JESSI, ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER));
    ret_vec.push(game_a4(PAEDAGOGISCH, LUISE, LENA, TEAM_BRIDE, DZANA, VERENA));
    ret_vec.push(game_a5(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, PAEDAGOGISCH, LUISE, LENA));
    ret_vec.push(game_a6(TEAM_BRIDE, DZANA, VERENA, HELLWACH, NIKO, JAKOB_HELLWACH));
    ret_vec.push(game_a7(TOBÖNKE, TOBIAS, SÖNKE, JESS_RON, ARON, JESSI));
    ret_vec.push(game_a8(COON_AND_FRIENDS, CHRIS, DAVID, JGA2, KATI_JGA, SOPHIE));
    ret_vec.push(game_a9(PAEDAGOGISCH, LUISE, LENA, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_a10(JESS_RON, ARON, JESSI, JGA2, KATI_JGA, SOPHIE));
    ret_vec.push(game_a11(HELLWACH, JAKOB_HELLWACH, NIKO, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(game_a12(TEAM_BRIDE, DZANA, VERENA, ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER));
    ret_vec.push(game_a13(COON_AND_FRIENDS, CHRIS, DAVID,JESS_RON, ARON, JESSI));
    ret_vec.push(game_a14(TOBÖNKE, TOBIAS, SÖNKE, TEAM_BRIDE, DZANA, VERENA));
    ret_vec.push(game_a15(JGA2, KATI_JGA, SOPHIE, PAEDAGOGISCH, LUISE, LENA));
    ret_vec.push(game_a16(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, HELLWACH, NIKO, JAKOB_HELLWACH));
    ret_vec.push(game_a17(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_a18(PAEDAGOGISCH, LUISE,LENA, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(game_a19(TEAM_BRIDE, DZANA, VERENA, JGA2, SOPHIE, KATI_JGA));
    ret_vec.push(game_a20(HELLWACH, JAKOB_HELLWACH, NIKO,JESS_RON, ARON, JESSI));
    ret_vec.push(game_a21(COON_AND_FRIENDS, CHRIS, DAVID,TEAM_BRIDE, DZANA, VERENA));
    ret_vec.push(game_a22(JGA2, KATI_JGA, SOPHIE, ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER));
    ret_vec.push(game_a23(TOBÖNKE, TOBIAS, SÖNKE, HELLWACH, NIKO, JAKOB_HELLWACH));
    ret_vec.push(game_a24(JESS_RON, ARON, JESSI, PAEDAGOGISCH, LUISE, LENA));
    ret_vec.push(game_a25(TEAM_BRIDE, DZANA, VERENA, JESS_RON, ARON, JESSI));
    ret_vec.push(game_a26(HELLWACH, JAKOB_HELLWACH, NIKO,PAEDAGOGISCH, LUISE, LENA));
    ret_vec.push(game_a27(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(game_a28(TOBÖNKE, TOBIAS, SÖNKE, JGA2, KATI_JGA, SOPHIE));
    // Side B
    ret_vec.push(game_b1(JBG, JOELINA, SARAH_BENKE, JGA1, MICHELLE, MARIAM));
    ret_vec.push(game_b2(TIMMY, FLO_TIMMY, ARNE, DOS_BROS, FLO, SEBI));
    ret_vec.push(game_b3(PEGELPROFIS, RENE, LYNN, JGA3, LISA, XATUN));
    ret_vec.push(game_b4(DOS_BROS, SEBI, FLO, PEGELPROFIS, RENE, LYNN));
    ret_vec.push(game_b5(JGA1, MARIAM, MICHELLE, TIMMY, ARNE, FLO_TIMMY));
    ret_vec.push(game_b6(MR_MRS_SMITH, JEROME, JACQUELINE, JBG, JOELINA, SARAH_BENKE));
    ret_vec.push(game_b7(TIMMY, FLO_TIMMY, ARNE, MR_MRS_SMITH, JEROME, JACQUELINE));
    ret_vec.push(game_b8(PEGELPROFIS, RENE, LYNN, JGA1, MICHELLE, MARIAM));
    ret_vec.push(game_b9(JGA3, LISA, XATUN, DOS_BROS, FLO, SEBI));
    ret_vec.push(game_b10(JGA1, MARIAM, MICHELLE, JGA3, LISA, XATUN));
    ret_vec.push(game_b11(MR_MRS_SMITH, JEROME, JACQUELINE, PEGELPROFIS, RENE, LYNN));
    ret_vec.push(game_b12(JBG, JOELINA, SARAH_BENKE, TIMMY, ARNE, FLO_TIMMY));
    ret_vec.push(game_b13(PEGELPROFIS, RENE, LYNN, JBG, JOELINA, SARAH_BENKE));
    ret_vec.push(game_b14(JGA3, LISA, XATUN, MR_MRS_SMITH, JEROME, JACQUELINE));
    ret_vec.push(game_b15(DOS_BROS, FLO, SEBI, JGA1, MICHELLE, MARIAM));
    ret_vec.push(game_b16(MR_MRS_SMITH, JEROME, JACQUELINE, DOS_BROS, SEBI, FLO));
    ret_vec.push(game_b17(JBG, JOELINA, SARAH_BENKE, JGA3, LISA, XATUN));
    ret_vec.push(game_b18(TIMMY, FLO_TIMMY, ARNE, PEGELPROFIS, RENE, LYNN));
    ret_vec.push(game_b19(JGA3, LISA, XATUN, TIMMY, ARNE, FLO_TIMMY));
    ret_vec.push(game_b20(JBG, JOELINA, SARAH_BENKE, DOS_BROS, FLO, SEBI));
    ret_vec.push(game_b21(JGA1, MICHELLE, MARIAM, MR_MRS_SMITH, JEROME, JACQUELINE));
    // Double elimination
    ret_vec.push(elim_1(COON_AND_FRIENDS, CHRIS, DAVID, JGA1, MARIAM, MICHELLE));
    ret_vec.push(elim_2(MR_MRS_SMITH, JEROME, JACQUELINE,HELLWACH,NIKO, JAKOB_HELLWACH));
    ret_vec.push(elim_3(TIMMY, FLO_TIMMY, ARNE, PAEDAGOGISCH, LUISE, LENA));
    ret_vec.push(elim_4(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, DOS_BROS, FLO, SEBI));
    ret_vec.push(elim_5(JGA1, MICHELLE, MARIAM, HELLWACH, JAKOB_HELLWACH, NIKO));
    ret_vec.push(elim_6(PAEDAGOGISCH, LENA, LUISE, ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER));
    ret_vec.push(elim_7(COON_AND_FRIENDS, CHRIS, DAVID, MR_MRS_SMITH, JEROME, JACQUELINE));
    ret_vec.push(elim_8(TIMMY, ARNE, FLO_TIMMY, DOS_BROS, SEBI, FLO));
    ret_vec.push(elim_9(TIMMY, FLO_TIMMY, ARNE, HELLWACH, NIKO, JAKOB_HELLWACH));
    ret_vec.push(elim_10(ALTBIERBURSCHEN, MAXI, JAKOB_ALTBIER, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(elim_11(MR_MRS_SMITH, JEROME, JACQUELINE, DOS_BROS, FLO, SEBI));
    ret_vec.push(elim_12(TIMMY, ARNE, FLO_TIMMY, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(elim_13(MR_MRS_SMITH, JEROME, JACQUELINE, COON_AND_FRIENDS, CHRIS, DAVID));
    ret_vec.push(elim_14(DOS_BROS, FLO, SEBI, MR_MRS_SMITH, JEROME, JACQUELINE));
    ret_vec.push(elim_15(MR_MRS_SMITH, JEROME, JACQUELINE, DOS_BROS, SEBI, FLO));

    convert_first_throw_games(&mut ret_vec);
    ret_vec
}
/*
pub fn elim_(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(), Some()), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}*/
pub fn elim_15(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(5), Some(7), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,3,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_14(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((None, None, Some(6), Some(6)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![3,1,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_13(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(10), Some(10), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,2,2,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_12(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None, Some(7), Some(7)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_11(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None, Some(5), Some(5)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_10(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 0), ARC::schluck(&left_2, 0), ARC::schluck(&right_2, 2), ARC::finish(&right_1, 4), ARC::finish(&right_2,4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_9(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(10), Some(10), None, Some(9)), vec![ARC::schluck(&right_1,1)], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,3,1,1,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_8(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None,Some(7), Some(7)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_7(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None,Some(7), Some(7)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((None, None, Some(12), Some(12)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![8, 1, 1, 1, 1, 1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 4), ARC::beer(&left_2, 11), ARC::finish(&right_1, 14), ARC::finish(&right_2, 14)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![3,2,1,1,3,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(4), None, Some(5), Some(7)), vec![ARC::schluck(&left_1, 0)], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,1,1,2,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(7), Some(7), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,2,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(7), Some(7), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,7]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn elim_1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(8), Some(8), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,1,1,2,1,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b21(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None,Some(9), Some(9)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,3,1,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b20(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 11), ARC::schluck(&left_1, 18), ARC::schluck(&right_2,18), ARC::finish(&right_2, 19)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![5,1,2,1,2,1,6,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b19(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 2), ARC::schluck(&left_1, 2), ARC::finish(&right_1, 2), ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b18(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(13), Some(11), None, None), vec![ARC::schluck(&right_2, 0), ARC::schluck(&left_1,3)], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,2,4,1,2,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b17(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 7), ARC::schluck(&left_2, 7), ARC::schluck(&right_2, 7), ARC::finish(&right_1, 14),ARC::finish(&left_1, 17), ARC::finish(&left_2,17)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![7,3,2,1,1,1,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b16(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(5), Some(11),None,None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,2,1,2,2,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b15(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(6), Some(11),None,None), vec![ARC::schluck(&right_1, 1), ARC::schluck(&right_2,11)], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,3,1,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b14(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None,Some(7), Some(9)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,3,1,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b13(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(16), Some(19), None, None), vec![ARC::schluck(&right_1, 16)], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,6,1,1,1,3,2,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b12(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(14), Some(14), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![5,1,1,2,2,2,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b11(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(9), Some(9), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,4,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b10(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_2, 0), ARC::schluck(&right_1, 0), ARC::schluck(&left_1, 7), ARC::schluck(&right_2, 17), ARC::finish(&left_1, 17), ARC::finish(&left_2,17)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, vec![false, false,false,true,false,false,false,false,false,true,false,false,false,false,false,true,true,true], additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b9(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((None, None,Some(7), Some(7)), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![3,1,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b8(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = ARC::all_additionals((Some(24), Some(20), None, None), vec![], &left_1, &left_2, &right_1, &right_2);
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, vec![false,false,false,false,true,true,false,false,false,false,false,true,true,false,false,false,false,false,true,false,true,false,false,false,true], additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b7(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = ARC::all_additionals((Some(7), Some(7), None, None), vec![], &left_1, &left_2, &right_1, &right_2); //vec![ARC::finish(&left_1, 7),ARC::finish(&left_2, 7)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,2,4]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 11),ARC::finish(&left_2, 11)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,2,1,1,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_2, 0), ARC::finish(&right_1, 6),ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_2, 3),ARC::finish(&left_1, 13)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,3,4,1,1,1,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_1, 6), ARC::schluck(&right_1, 20), ARC::finish(&left_1, 23),ARC::finish(&left_2, 23)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,2,1,1,2,9,2,2,1,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 0), ARC::finish(&left_1, 5),ARC::finish(&left_2, 5)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![4,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_b1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1, 6), ARC::schluck(&right_2, 6), ARC::finish(&right_2, 15), ARC::finish(&left_1, 16),ARC::finish(&right_1, 17)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![6,1,6,1,1,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a28(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 8),ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,1,3,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a27(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 7),ARC::finish(&right_2, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,2,2,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a26(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 17), ARC::finish(&left_1, 20),ARC::finish(&left_2, 20)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,1,1,2,1,1,6,1,2,1,2,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a25(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_2, 8),ARC::finish(&left_1, 12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,4,2,1,3,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a24(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1,11),ARC::finish(&left_2, 12),ARC::finish(&right_2, 19)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,4,1,2,2,2,1,2,3,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a23(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_1, 3), ARC::finish(&right_2, 7),ARC::finish(&left_1, 8),ARC::finish(&right_1, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![3,1,1,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a22(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_1, 6),ARC::finish(&right_2, 10),ARC::schluck(&left_1,15), ARC::finish(&right_1, 15)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![5,4,1,1,4,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a21(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_2, 6),ARC::finish(&right_1, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,3,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a20(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_2, 0), ARC::schluck(&right_1,1), ARC::beer(&right_1,3),ARC::finish(&left_1, 6),ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,4,3,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}

pub fn game_a19(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_2, 8),ARC::finish(&left_1, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,1,5,1,1,1]), additionals, left_began);
    Game{ result, match_number: 119, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a18(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 9),ARC::finish(&right_2, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![3,1,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 118, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a17(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 4),ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,5]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a16(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&left_1, 1), ARC::schluck(&right_1, 1),ARC::schluck(&right_2, 1), ARC::finish(&left_2, 1),ARC::schluck(&right_1, 7), ARC::finish(&left_1, 7)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,3,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a15(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 10),ARC::finish(&right_2, 12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,2,2,1,1,2,2,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a14(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 7),ARC::finish(&right_2, 7)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,3,1]), additionals, left_began);
    Game{ result, match_number: 114, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a13(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_2, 1), ARC::finish(&left_1, 8),ARC::finish(&left_2, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,2,5]), additionals, left_began);
    Game{ result, match_number: 113, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a12(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::beer(&right_2, 7), ARC::beer(&right_1, 9), ARC::finish(&right_2, 19),ARC::finish(&left_1, 20), ARC::schluck(&left_2, 20), ARC::finish(&right_1, 20)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2,vec![false,false,true,true,true,true,false,true,false,true,false,true,false,true,false,false,false,false,false,true,true], additionals, left_began);
    Game{ result, match_number: 112, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a11(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 12), ARC::finish(&right_2, 13),ARC::finish(&left_2, 14)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,4,2,1,1,1,3]), additionals, left_began);
    Game{ result, match_number: 111, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a10(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_2, 12), ARC::schluck(&right_1,12), ARC::finish(&left_1, 16),ARC::schluck(&right_2, 25),ARC::finish(&left_2, 28)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,9,1,3,1,2,1,3,1,1,2,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a9(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 15),ARC::finish(&left_2, 17)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![5,2,1,2,3,1,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a8(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_2, 4),ARC::schluck(&left_2, 15),ARC::finish(&left_2, 17),ARC::finish(&left_1, 19)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![4,1,1,1,8,1,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a7(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1, 3),ARC::finish(&left_1, 12),ARC::finish(&left_2, 14)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,1,1,1,2,5,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&left_1, 3), ARC::schluck(&right_1, 4), ARC::schluck(&right_2,4),ARC::finish(&left_2, 4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_1,0), ARC::schluck(&left_1,2), ARC::finish(&left_2, 10),ARC::finish(&left_1, 12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,4,1,4,3]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 4),ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![3,1,1,1,1,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::schluck(&right_1, 2), ARC::schluck(&left_1, 2) ,ARC::finish(&left_1, 3),ARC::finish(&right_2, 10), ARC::finish(&right_1,10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,4,1,2,1,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
pub fn game_a2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 4),ARC::finish(&left_2, 4)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![3,1,1]), additionals, left_began);
    Game{ result, match_number: 102, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}

pub fn game_a1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&right_1,1), ARC::finish(&left_1, 10),ARC::finish(&left_2, 10)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,2,2,2,2]), additionals, left_began);
    Game{ result, match_number: 101, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None }
}
