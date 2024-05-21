use std::collections::HashSet;
use crate::data::{Game, Team, TeamMember};
use crate::team_player_data::TEAM_INVALID;

pub fn player_in_team(player_id: u32, team: &Team) -> bool {
    return team.member_1.id == player_id || team.member_2.id == player_id;
}

pub fn team_from_player(player_id: u32, teams: &Vec<Team>) -> &Team {
    for team in teams {
        if team.member_1.id == player_id || team.member_2.id == player_id {
            return team;
        }
    }
    &TEAM_INVALID
}

pub fn team_id_from_player(player_id: u32, teams: &Vec<Team>) -> u32 {
    team_from_player(player_id, teams).id
}

pub fn teams_from_games(games : &Vec<Game>)-> Vec<Team>{
    let mut set : HashSet<Team> = HashSet::new();
    for game in games{
        set.insert(game.left_team.clone());
        set.insert(game.right_team.clone());
    }
    set.iter().cloned().collect()
}
pub fn players_from_games(games : &Vec<Game>)-> Vec<TeamMember>{
    let mut set : HashSet<TeamMember> = HashSet::new();
    for game in games{
        set.insert(game.left_1.clone());
        set.insert(game.left_2.clone());
        set.insert(game.right_1.clone());
        set.insert(game.right_2.clone());
    }
    set.iter().cloned().collect()
}

pub fn team_name_from_id(team_id: u32, teams: &Vec<Team>) -> &str {
    for team in teams {
        if team.id == team_id {
            return team.name;
        }
    }
    "Not Found"
}

pub fn player_name_from_id(player_id: u32, players: &Vec<TeamMember>) -> &str {
    for player in players {
        if player.id == player_id {
            return player.name;
        }
    }
    "Not Found"
}

pub fn name_from_id(id: u32, teams: &Vec<Team>, players: &Vec<TeamMember>) -> &'static str {
    for team in teams {
        if team.id == id {
            return team.name;
        }
    }
    for player in players {
        if player.id == id {
            return player.name;
        }
    }
    "Name not found"
}
pub fn print_line_break(width: usize) {
    println!("{:-<width$}", "-")
}

pub fn player_is_in_game(game: &Game, player: &TeamMember) -> bool {
    let player_ids = vec![game.left_1.id, game.left_2.id, game.right_1.id, game.right_2.id];
    return player_ids.contains(&player.id);
}
pub fn team_is_in_game(game: &Game, team: &Team) -> bool {
    let team_ids = vec![game.left_team.id, game.right_team.id];
    return team_ids.contains(&team.id);
}

#[cfg(test)]
pub mod test{
    use crate::data::{ARC, bool_vec_from_int, create_normal_rounds_left_right, Game, results_from_additionals, Team};

    pub fn game_2nd_finish(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![ARC::finish(&left_team.member_1, 2), ARC::finish(&left_team.member_2, 2)];
        let result = results_from_additionals(&additionals, left_began, &left_team);
        let rounds = create_normal_rounds_left_right(&left_team.member_1, &left_team.member_2, &right_team.member_1, &right_team.member_2, bool_vec_from_int(true, vec![3]), additionals, left_began);
        Game { result, match_number: 12, left_team: left_team.clone(), left_1: left_team.member_1, left_2: left_team.member_2, right_team: right_team.clone(), right_1:right_team.member_1, right_2: right_team.member_2, rounds }
    }
    pub fn game_2nd_finish_enemy_miss(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![ARC::finish(&left_team.member_1, 2), ARC::finish(&left_team.member_2, 2)];
        let result = results_from_additionals(&additionals, left_began, &left_team);
        let rounds = create_normal_rounds_left_right(&left_team.member_1, &left_team.member_2, &right_team.member_1, &right_team.member_2,vec![true, false,true], additionals, left_began);
        Game { result, match_number: 12, left_team: left_team.clone(), left_1: left_team.member_1, left_2: left_team.member_2, right_team: right_team.clone(), right_1:right_team.member_1, right_2: right_team.member_2, rounds }
    }

    pub fn game_3rd_finish(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![ARC::finish(&left_team.member_1, 4), ARC::finish(&left_team.member_2, 4)];
        let result = results_from_additionals(&additionals, left_began, &left_team);
        let rounds = create_normal_rounds_left_right(&left_team.member_1, &left_team.member_2, &right_team.member_1, &right_team.member_2, bool_vec_from_int(true, vec![5]), additionals, left_began);
        Game { result, match_number: 12, left_team: left_team.clone(), left_1: left_team.member_1, left_2: left_team.member_2, right_team: right_team.clone(), right_1:right_team.member_1, right_2: right_team.member_2, rounds }
    }

    pub fn game_1st_finish_2straf(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![ARC::schluck(&right_team.member_1, 1), ARC::schluck(&right_team.member_2, 1), ARC::finish(&left_team.member_1, 1), ARC::finish(&left_team.member_2, 1)];
        let result = results_from_additionals(&additionals, left_began, &left_team);
        let rounds = create_normal_rounds_left_right(&left_team.member_1, &left_team.member_2, &right_team.member_1, &right_team.member_2, bool_vec_from_int(true, vec![2]), additionals, left_began);
        Game { result, match_number: 12, left_team: left_team.clone(), left_1: left_team.member_1, left_2: left_team.member_2, right_team: right_team.clone(), right_1:right_team.member_1, right_2: right_team.member_2, rounds }
    }
}