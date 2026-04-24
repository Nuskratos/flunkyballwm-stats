use crate::calc::penalties_data::{Penalties, PenaltiesStatistics, PlayerPenalties, TeamPenalties};
use crate::data::AdditionalType::{STRAFBIER, STRAFSCHLUCK};
use crate::data::{Game, Team, TeamMember};
use crate::util::{team_from_player, team_id_from_player};
use std::collections::HashMap;

pub fn calculate_amount_of_penalties(games: &Vec<Game>) -> PenaltiesStatistics {
    let mut team_map: HashMap<Team, Penalties> = HashMap::new(); // Implementation has a map though it might not need it anymore after switching (u32) to team&player
    let mut player_map: HashMap<TeamMember, Penalties> = HashMap::new();
    for game in games {
        team_map
            .entry(game.left_team.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        team_map
            .entry(game.right_team.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        player_map
            .entry(game.left_1.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        player_map
            .entry(game.left_2.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        player_map
            .entry(game.right_1.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        player_map
            .entry(game.right_2.to_owned())
            .and_modify(|x| x.games += 1)
            .or_insert(Penalties::initial());
        for round in &game.rounds {
            for add in &round.additionals {
                match add.kind {
                    STRAFSCHLUCK => {
                        team_map
                            .entry(team_from_player(add.source.id(), game).to_owned())
                            .and_modify(|x| x.schlucke += 1)
                            .or_insert(Penalties::create_schluck());
                        player_map
                            .entry(add.source.to_owned())
                            .and_modify(|x| x.schlucke += 1)
                            .or_insert(Penalties::create_schluck());
                    }
                    STRAFBIER => {
                        team_map
                            .entry(team_from_player(add.source.id(), game).to_owned())
                            .and_modify(|x| x.beers += 1)
                            .or_insert(Penalties::create_beer());
                        player_map
                            .entry(add.source.to_owned())
                            .and_modify(|x| x.beers += 1)
                            .or_insert(Penalties::create_beer());
                    }
                    _ => {}
                }
            }
        }
    }
    let mut team_vec: Vec<TeamPenalties> = Vec::new();
    for team in team_map {
        team_vec.push(TeamPenalties {
            team: team.0,
            stats: team.1,
        });
    }
    team_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    let mut player_vec: Vec<PlayerPenalties> = Vec::new();
    for player in player_map {
        player_vec.push(PlayerPenalties {
            player: player.0,
            stats: player.1,
        });
    }
    player_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    PenaltiesStatistics {
        teams: team_vec,
        players: player_vec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2};
    use crate::util::test::{
        game_1st_finish_2straf, game_2nd_finish_right_began, game_5th_finish_strafbeer,
    };
    use approx::assert_relative_eq;
    use float_cmp::approx_eq;
    
    #[test]
    fn test_single_game(){
        let games = vec![game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2)];
        let result = calculate_amount_of_penalties(&games);
        let t1_1 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM1.member_1.name())
            .unwrap();
        let t1_2 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM1.member_2.name())
            .unwrap();
        let t2_1 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM2.member_1.name())
            .unwrap();
        let t2_2 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM2.member_2.name())
            .unwrap();

        let team1 = result.teams.iter().find(|x| x.team.name() == TEST_TEAM1.name()).unwrap();
        let team2 = result.teams.iter().find(|x| x.team.name() == TEST_TEAM2.name()).unwrap();

        assert_relative_eq!(t1_1.stats.bpg(), 0f32 / 1f32);
        assert_relative_eq!(t1_1.stats.spg(), 0f32 / 1f32);

        assert_relative_eq!(t1_2.stats.bpg(), 0.0);
        assert_relative_eq!(t1_2.stats.spg(), 0f32 / 1f32);

        assert_relative_eq!(t2_1.stats.bpg(), 0.0);
        assert_relative_eq!(t2_1.stats.spg(), 1f32 / 1f32);

        assert_relative_eq!(t2_2.stats.bpg(), 0.0);
        assert_relative_eq!(t2_2.stats.spg(), 1f32 / 1f32);

        assert_relative_eq!(team1.stats.bpg(), 0f32 / 1f32);
        assert_relative_eq!(team1.stats.spg(), 0f32 / 1f32);

        assert_relative_eq!(team2.stats.bpg(), 0.0);
        assert_relative_eq!(team2.stats.spg(), 2f32 / 1f32);
        
    }
    #[test]
    fn test_calculate_amount_of_penalties() {
        let games = vec![
            game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2),
            game_1st_finish_2straf(TEST_TEAM1, TEST_TEAM2),
            game_5th_finish_strafbeer(TEST_TEAM1, TEST_TEAM2),
        ];
        let result = calculate_amount_of_penalties(&games);
        let t1_1 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM1.member_1.name())
            .unwrap();
        let t1_2 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM1.member_2.name())
            .unwrap();
        let t2_1 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM2.member_1.name())
            .unwrap();
        let t2_2 = result
            .players
            .iter()
            .find(|x| x.player.named_entity.name == TEST_TEAM2.member_2.name())
            .unwrap();

        let team1 = result.teams.iter().find(|x| x.team.name() == TEST_TEAM1.name()).unwrap();
        let team2 = result.teams.iter().find(|x| x.team.name() == TEST_TEAM2.name()).unwrap();

        assert_relative_eq!(t1_1.stats.bpg(), 1f32 / 3f32);
        assert_relative_eq!(t1_1.stats.spg(), 1f32 / 3f32);

        assert_relative_eq!(t1_2.stats.bpg(), 0.0);
        assert_relative_eq!(t1_2.stats.spg(), 1f32 / 3f32);

        assert_relative_eq!(t2_1.stats.bpg(), 0.0);
        assert_relative_eq!(t2_1.stats.spg(), 3f32 / 3f32);

        assert_relative_eq!(t2_2.stats.bpg(), 0.0);
        assert_relative_eq!(t2_2.stats.spg(), 2f32 / 3f32);

        assert_relative_eq!(team1.stats.bpg(), 1f32 / 3f32);
        assert_relative_eq!(team1.stats.spg(), 2f32 / 3f32);

        assert_relative_eq!(team2.stats.bpg(), 0.0);
        assert_relative_eq!(team2.stats.spg(), 5f32 / 3f32);
    }
}
