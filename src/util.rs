use crate::data::{Game, Team, TeamMember};
use crate::team_player_data::TEAM_INVALID;
use csv::Writer;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn player_in_team(player_id: u32, team: &Team) -> bool {
    return team.member_1.id() == player_id || team.member_2.id() == player_id;
}

pub fn team_from_player(player_id: u32, game: &Game) -> &Team {
    let teams = vec![&game.left_team, &game.right_team];
    for team in teams {
        if team.member_1.id() == player_id || team.member_2.id() == player_id {
            return team;
        }
    }
    &TEAM_INVALID
}

pub fn team_id_from_player(player_id: u32, game: &Game) -> u32 {
    team_from_player(player_id, game).id()
}

pub fn teams_from_games(games: &Vec<Game>) -> Vec<Team> {
    let mut set: HashSet<Team> = HashSet::new();
    for game in games {
        set.insert(game.left_team.clone());
        set.insert(game.right_team.clone());
    }
    set.iter().cloned().collect()
}
pub fn players_from_games(games: &Vec<Game>) -> Vec<TeamMember> {
    let mut set: HashSet<TeamMember> = HashSet::new();
    for game in games {
        set.insert(game.left_1.clone());
        set.insert(game.left_2.clone());
        set.insert(game.right_1.clone());
        set.insert(game.right_2.clone());
    }
    set.iter().cloned().collect()
}

pub fn team_name_from_id(team_id: u32, teams: &Vec<Team>) -> &str {
    for team in teams {
        if team.id() == team_id {
            return team.name();
        }
    }
    "Not Found"
}

pub fn player_name_from_id(player_id: u32, players: &Vec<TeamMember>) -> &str {
    for player in players {
        if player.id() == player_id {
            return player.name();
        }
    }
    "Not Found"
}

pub fn print_line_break(width: usize) {
    println!("{:-<width$}", "-")
}

pub fn player_is_in_game(game: &Game, player: &TeamMember) -> bool {
    let player_ids = vec![
        game.left_1.id(),
        game.left_2.id(),
        game.right_1.id(),
        game.right_2.id(),
    ];
    return player_ids.contains(&player.id());
}
pub fn team_is_in_game(game: &Game, team: &Team) -> bool {
    let team_ids = vec![game.left_team.id(), game.right_team.id()];
    return team_ids.contains(&team.id());
}

pub struct OpenedWriter {
    pub writer: Writer<File>,
    pub file_exists: bool,
}

pub fn open_writer(filename: String) -> OpenedWriter {
    let path = Path::new(&filename);
    let file_exists = path.exists();
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Couldn't open file");
    let mut writer = csv::Writer::from_writer(file);
    OpenedWriter {
        writer,
        file_exists,
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::{
        bool_vec_from_int, create_normal_rounds_left_right, results_from_additionals, Game, Team,
        ARC,
    };

    pub fn game_2nd_finish(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![
            ARC::finish(&left_team.member_1, 2),
            ARC::finish(&left_team.member_2, 2),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            bool_vec_from_int(true, vec![3]),
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }
    pub fn game_2nd_finish_enemy_miss(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![
            ARC::finish(&left_team.member_1, 2),
            ARC::finish(&left_team.member_2, 2),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            vec![true, false, true],
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }

    pub fn game_3rd_finish(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![
            ARC::finish(&left_team.member_1, 4),
            ARC::finish(&left_team.member_2, 4),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            bool_vec_from_int(true, vec![5]),
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }

    pub fn game_1st_finish_2straf(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![
            ARC::schluck(&right_team.member_1, 1),
            ARC::schluck(&right_team.member_2, 1),
            ARC::finish(&left_team.member_1, 1),
            ARC::finish(&left_team.member_2, 1),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            bool_vec_from_int(true, vec![2]),
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }
    pub fn game_2nd_finish_right_began(left_team: Team, right_team: Team) -> Game {
        let left_began = false;
        let additionals = vec![
            ARC::finish(&right_team.member_1, 2),
            ARC::finish(&right_team.member_2, 2),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            bool_vec_from_int(true, vec![3]),
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }
    pub fn game_5th_finish_strafbeer(left_team: Team, right_team: Team) -> Game {
        let left_began = true;
        let additionals = vec![
            ARC::beer(&left_team.member_1, 0),
            ARC::schluck(&left_team.member_2, 0),
            ARC::finish(&left_team.member_2, 2),
            ARC::schluck(&left_team.member_1, 2),
            ARC::schluck(&right_team.member_1, 4),
            ARC::finish(&left_team.member_1, 4),
        ];
        let result = results_from_additionals(&additionals, &left_team);
        let rounds = create_normal_rounds_left_right(
            &left_team.member_1,
            &left_team.member_2,
            &right_team.member_1,
            &right_team.member_2,
            bool_vec_from_int(true, vec![5]),
            additionals,
            left_began,
        );
        Game {
            result,
            match_number: 12,
            left_team: left_team.clone(),
            left_1: left_team.member_1,
            left_2: left_team.member_2,
            right_team: right_team.clone(),
            right_1: right_team.member_1,
            right_2: right_team.member_2,
            rounds,
        }
    }
}
