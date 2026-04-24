use crate::calc::rock_paper_scissors_data::{RockPaperScissorSingleStats, RockPaperScissorStats};
use crate::data;
use crate::data::{Entity, Game};
use crate::util::team_from_player;
use std::collections::HashMap;

pub fn calculate_rock_paper_scissors(games: &Vec<Game>) -> RockPaperScissorStats {
    let mut scores: HashMap<Entity, RockPaperScissorSingleStats> = HashMap::new();
    for game in games {
        let winner: Entity = Entity::Player(game.rounds.first().unwrap().thrower);
        let loser: Entity = Entity::Player(game.rounds.first().unwrap().runner);
        let winner_team: Entity =
            Entity::Team(team_from_player(winner.named().id, game).to_owned());
        let loser_team: Entity =
            Entity::Team(team_from_player(loser.named().id, game).to_owned());
        scores
            .entry(winner.to_owned())
            .or_insert(RockPaperScissorSingleStats::default(&winner))
            .won(&loser);
        scores
            .entry(loser.to_owned())
            .or_insert(RockPaperScissorSingleStats::default(&loser))
            .lost(&winner);
        scores
            .entry(winner_team.to_owned())
            .or_insert(RockPaperScissorSingleStats::default(&winner_team))
            .won(&loser_team);
        scores
            .entry(loser_team.to_owned())
            .or_insert(RockPaperScissorSingleStats::default(&loser_team))
            .lost(&winner_team);
    }
    let mut data: Vec<_> = scores.into_values().collect();
    data.sort();
    RockPaperScissorStats { data }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use crate::calc::rock_paper_scissors_calc::calculate_rock_paper_scissors;
    use crate::calc::rock_paper_scissors_data::RockPaperScissorSingleStats;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2, TEST_TEAM3};
    use crate::util::test::{game_2nd_finish, game_2nd_finish_right_began, game_3rd_finish};

    #[test]
    fn rock_paper_scissors_calc() {
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_3rd_finish(TEST_TEAM1, TEST_TEAM2),
            game_3rd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
        ];
        let resulting_values = calculate_rock_paper_scissors(&games);
        let stats_member1 = resulting_values.data.iter().find(|x| x.entity.named().name == TEST_TEAM1.member_1.named_entity.name).unwrap();
        let stats_member2 = resulting_values.data.iter().find(|x| x.entity.named().name == TEST_TEAM2.member_1.named_entity.name).unwrap();
        let stats_member3 = resulting_values.data.iter().find(|x| x.entity.named().name == TEST_TEAM3.member_1.named_entity.name).unwrap();
        assert!(approx_eq!(f32, stats_member1.stats.win_percentage(), 3f32 / 4f32 *100.0));
        assert!(approx_eq!(f32, stats_member2.stats.win_percentage(), 0.0));
        assert!(approx_eq!(f32, stats_member3.stats.win_percentage(), 1f32 / 2f32 *100.0));

        let member1_nemesis = stats_member1.worst_matchup();
        assert_eq!(member1_nemesis.entity.named().name, TEST_TEAM3.member_1.named_entity.name);
        let member1_best = stats_member1.best_matchup();
        assert_eq!(member1_best.entity.named().name, TEST_TEAM2.member_1.named_entity.name);
    }

    #[test]
    fn rock_paper_test_best_close(){
        // want to test if 2/3 is worse than 10/16
        let games = vec![
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM2),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
            game_2nd_finish_right_began(TEST_TEAM1, TEST_TEAM3),
        ];
        let resulting_values = calculate_rock_paper_scissors(&games);
        let stats_member1 = resulting_values.data.iter().find(|x| x.entity.named().name == TEST_TEAM1.member_1.named_entity.name).unwrap();
        let member1_best = stats_member1.best_matchup();
        assert_eq!(member1_best.entity.named().name, TEST_TEAM3.member_1.named_entity.name);
    }
}
