use std::collections::HashMap;
use crate::calc::accuracy_data::{Accuracy, EnemyAccuracy};
use crate::data::{Game, NamedEntity, Team, TeamMember};
use crate::util::{team_from_player, team_id_from_player};

pub fn calculate_throwing_accuracy(games: &Vec<Game>, teams: &Vec<Team>, players: &Vec<TeamMember>) -> Vec<Accuracy>{
    let mut throws = 0;
    let mut hits = 0;
    let mut player_scores: HashMap<NamedEntity,Accuracy> = HashMap::new();
    let mut team_scores : HashMap<NamedEntity,Accuracy> = HashMap::new();

    for game in games {
        for round in &game.rounds {
            let playerAccuracy = player_scores.entry(round.thrower.named_entity.to_owned()).or_insert(Accuracy::default(round.thrower.named_entity.to_owned()));
            let teamAccuracy = team_scores.entry(team_from_player(round.thrower.id(), game).named_entity.to_owned()).or_insert(Accuracy::default(round.thrower.named_entity.to_owned()));
            throws = throws + 1;
            playerAccuracy.throws += 1;
            teamAccuracy.throws += 1;
            if round.hit {
                hits = hits + 1;
                playerAccuracy.hits += 1;
                teamAccuracy.hits += 1;
            }
        }
    }

    let mut result_vec: Vec<Accuracy> = Vec::new();
    for score in team_scores {
        result_vec.push(score.1);
    }
    for score in player_scores {
        result_vec.push(score.1);
    }
    result_vec.push(Accuracy{throws:throws, hits:hits, named_entity:NamedEntity{name:"Average", alias:"Average", id:999 }});
    result_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    result_vec.reverse();
    return result_vec;
}


pub fn calc_enemy_accuracy(games: &Vec<Game>) -> EnemyAccuracy{
    let mut enemy_accuracy: HashMap<&Team, Accuracy> = HashMap::new();
    for game in games {
        let first_enemy_stats = team_from_player(game.rounds.first().unwrap().thrower.id(), game);
        let second_enemy_stats = if &game.left_team == first_enemy_stats { &game.right_team } else { &game.left_team };
        for (ix, round) in game.rounds.iter().enumerate() {
            let passive_team = if ix % 2 == 0 { second_enemy_stats } else { first_enemy_stats };
            enemy_accuracy.entry(passive_team).and_modify(|x| x.add_throw(round.hit)).or_insert(Accuracy { named_entity: passive_team.named_entity.to_owned(), hits: if round.hit { 1 } else { 0 }, throws: 1 });
        }
    }
    let mut acc_vec: Vec< Accuracy> = Vec::new();
    for team in enemy_accuracy {
        acc_vec.push(team.1);
    }
    acc_vec.sort_by(|a, b| a.percentage().partial_cmp(&b.percentage()).unwrap());
    EnemyAccuracy{accuracies:acc_vec}
}