use crate::calc::chain_data::{
    ChainInformation, ChainStatistics, PlayerChainInformation, TeamChainInformation,
};
use crate::data::{Game, Team, TeamMember};
use crate::util::team_from_player;
use std::collections::HashMap;

pub fn calculate_hit_and_miss_chains_team_player(games: &Vec<Game>) -> ChainStatistics {
    let mut team_chain: HashMap<&Team, ChainInformation> = HashMap::new();
    let mut player_chain: HashMap<&TeamMember, ChainInformation> = HashMap::new();
    for game in games {
        for round in&game.rounds {
            let team = team_from_player(round.thrower.id(), game);
            team_chain
                .entry(team)
                .and_modify(|x| x.throw(round.hit))
                .or_insert(ChainInformation::create(round.hit));
            player_chain
                .entry(&round.thrower)
                .and_modify(|x| x.throw(round.hit))
                .or_insert(ChainInformation::create(round.hit));
        }
    }
    let mut team_vec: Vec<TeamChainInformation> = Vec::new();
    for team in team_chain {
        team_vec.push(TeamChainInformation {
            team: team.0.to_owned(),
            stats: team.1,
        });
    }
    team_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    let mut player_vec: Vec<PlayerChainInformation> = Vec::new();
    for player in player_chain {
        player_vec.push(PlayerChainInformation {
            player: player.0.to_owned(),
            stats: player.1,
        });
    }
    player_vec.sort_by(|a, b| a.stats.custom_cmp(&b.stats).unwrap());
    ChainStatistics {
        teams: team_vec,
        players: player_vec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_2nd_finish, game_2nd_finish_enemy_miss, game_3rd_finish};
    #[test]
    fn hit_and_miss_chains_team() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_enemy_miss(TEST_TEAM1, TEST_TEAM2),
            game_3rd_finish(TEST_TEAM1, TEST_TEAM2),
            game_3rd_finish(TEST_TEAM1, TEST_TEAM2),
        ];
        let data = calculate_hit_and_miss_chains_team_player(&games);
        let t1_m1 = data.players.iter().find(|x| x.player.named_entity.name == TEST_TEAM1.member_1.named_entity.name).unwrap();
        let t2_m1 = data.players.iter().find(|x| x.player.named_entity.name == TEST_TEAM2.member_1.named_entity.name).unwrap();

        assert_eq!(t1_m1.stats.total_hit, 6);
        assert_eq!(t1_m1.stats.total_miss, 0);
        assert_eq!(t2_m1.stats.total_hit, 2);
        assert_eq!(t2_m1.stats.total_miss, 1);
    }
}
