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
pub fn create_2026_01_spass() -> Vec<Game> {
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(game_1_1(DA_HAM_SIE, JEROME, BEEF, DER_WEDELMEDL, CHRIS_SINGLE_01, CHRIS_SINGLE_02));
    ret_vec.push(game_1_2(NATÜRLICH_BLOND, SEBI, LUISE, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_1_3(TOBÖNKE, TOBIAS, SÖNKE, DA_HAM_SIE, JEROME, BEEF));
    ret_vec.push(game_1_4(DER_WEDELMEDL, CHRIS_SINGLE_01, CHRIS_SINGLE_02, NATÜRLICH_BLOND, SEBI, LUISE));
    ret_vec.push(game_1_5(NATÜRLICH_BLOND, SEBI, LUISE, DA_HAM_SIE, JEROME, BEEF));
    ret_vec.push(game_1_6(DER_WEDELMEDL, CHRIS_SINGLE_01, CHRIS_SINGLE_02, TOBÖNKE, TOBIAS, SÖNKE));
    ret_vec.push(game_1_2nd(TOBÖNKE, TOBIAS, SÖNKE, NATÜRLICH_BLOND, SEBI, LUISE));
    convert_first_throw_games(&mut ret_vec);
    ret_vec
}
pub fn create_2026_02_spass() -> Vec<Game> {
    let mut ret_vec: Vec<Game> = Vec::new();
    ret_vec.push(game_2_1(NATÜRLICH_BLOND, SEBI, LUISE, CHROME, JEROME, CHRIS));
    ret_vec.push(game_2_2(ESSIN_ÖNKE, SOENKE_SINGLE_01, SOENKE_SINGLE_02, DER_SCHÖNE, BEEF,TOBIAS));
    ret_vec.push(game_2_3(DER_SCHÖNE, BEEF, TOBIAS, NATÜRLICH_BLOND, LUISE, SEBI));
    ret_vec.push(game_2_4(CHROME, JEROME, CHRIS, ESSIN_ÖNKE, SOENKE_SINGLE_01, SOENKE_SINGLE_02));
    ret_vec.push(game_2_5(ESSIN_ÖNKE, SOENKE_SINGLE_01, SOENKE_SINGLE_02, NATÜRLICH_BLOND, SEBI, LUISE));
    ret_vec.push(game_2_6(CHROME, JEROME, CHRIS, DER_SCHÖNE, BEEF, TOBIAS));
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
pub fn game_2_6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 4), ARC::finish(&right_2,9), ARC::finish(&right_1,13)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,2,1,1,1,1,3,1]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_2_5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 16), ARC::finish(&right_2,21), ARC::finish(&right_1,21)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![3,1,1,2,1,1,1,2,1,1,2,1,3,2]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_2_4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::schluck(&left_2, 10), ARC::schluck(&right_2,10), ARC::finish(&left_1, 10), ARC::finish(&right_2, 10), ARC::finish(&left_2,12)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,1,1,3,1,1,3,2]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_2_3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&left_1, 6), ARC::finish(&left_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,1,2,1,2]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_2_2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 8), ARC::beer(&right_2,8) ,ARC::finish(&left_1,15), ARC::finish(&right_2, 24)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![4,3,1,2,1,3,1,1,3,1,2,1,1,1]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_2_1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1, 4), ARC::finish(&right_2, 6)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,3]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: true }
}
pub fn game_1_2nd(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ ARC::finish(&left_1,10),ARC::finish(&right_1, 11),ARC::finish(&right_2, 11)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![2,1,1,2,3,3]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: true }
}
pub fn game_1_6(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ ARC::finish(&left_1,10),ARC::finish(&right_1, 11),ARC::finish(&right_2, 15)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,1,2,6,2,1]), additionals, left_began);
    Game{ result, match_number: 6, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false }
}
pub fn game_1_5(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ ARC::schluck(&right_1,0), ARC::finish(&right_1,8),ARC::finish(&left_1, 9),ARC::finish(&left_2, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,2,2,2,1,2]), additionals, left_began);
    Game{ result, match_number: 5, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: true }
}
pub fn game_1_4(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1,9),ARC::finish(&right_2, 9)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(false, vec![1,3,1,1,3,1]), additionals, left_began);
    Game{ result, match_number: 4, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false  }
}
pub fn game_1_3(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = true;
    let additionals = vec![ARC::finish(&right_1,5),ARC::finish(&right_2, 5)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![2,1,1,1,1]), additionals, left_began);
    Game{ result, match_number: 3, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false  }
}

pub fn game_1_2(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::finish(&right_1,6),ARC::finish(&right_2, 8)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![1,3,1,1,3]), additionals, left_began);
    Game{ result, match_number: 2, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false  }
}
pub fn game_1_1(left_team: Team, left_1:TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2:TeamMember)-> Game{
    let left_began = false;
    let additionals = vec![ARC::beer(&right_1,8), ARC::finish(&left_1, 11),ARC::finish(&left_2, 11)];
    let result = results_from_additionals(&additionals, &left_team);
    let rounds = create_normal_rounds_left_right(&left_1, &left_2, &right_1, &right_2, bool_vec_from_int(true, vec![3,4,3,1,1]), additionals, left_began);
    Game{ result, match_number: 1, left_team, left_1, left_2, right_1, right_2, right_team, rounds, special_first_throw: None, won_rps_but_decided_to_run: false  }
}
