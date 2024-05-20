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