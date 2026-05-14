use std::collections::HashMap;
use std::{error::Error, io, process};
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::Writer;
use crate::calc::accuracy_data::{Accuracy, EntityAccuracy, FirstThrowAccuracy};
use crate::calc::chain_calc::calculate_hit_and_miss_chains_team_player;
use crate::calc::drink_total_data::PlayerDrinkingSpeed;
use crate::calc::first_throw_data::FirstThrows;
use crate::calc::penalties_calc::calculate_amount_of_penalties;
use crate::calc::ppg_calc::calculate_amount_of_points_per_game;
use crate::calc::side_information_calc::calc_side_information;
use crate::calc::strafschluck_calc::calculate_strafschluck;
use crate::calc::throw_per_game_calc::calculate_throws_per_game;
use crate::data::{Game, NamedEntity, Team, TeamMember};
use crate::util::{print_line_break, team_from_player, team_id_from_player, team_name_from_id};
use crate::team_player_data::{AVERAGE_ENTITY, NAME_WIDTH}; // Used in printlines

pub fn percentage(divisor: usize, divident: usize) -> f32 { divisor as f32 / divident as f32 * 100.0 }

pub fn average(divisor: u32, dividend: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn wrong_way_average(dividend: u32, divisor: u32) -> f32 { divisor as f32 / dividend as f32 }

pub fn show_difference_for_first_throws(first_throws: &FirstThrowAccuracy, general_accuracy: &EntityAccuracy) -> (f32, f32) {
    let mut first_count = 0;
    let mut first_percentages = 0.0;
    let mut general_count = 0;
    let mut general_percentages = 0.0;
    for accuracy in general_accuracy.accuracies.iter() {
        if accuracy.named_entity.id < 1000{
            continue;
        }
        general_count += 1;
        general_percentages += accuracy.percentage();

        let first_accuracy =first_throws.accuracies.iter().find(|a| a.named_entity.id == accuracy.named_entity.id);
        if let Some(first_accuracy) = first_accuracy {
            let occurrences = first_accuracy.throws;
            first_count += occurrences;
            first_percentages += occurrences as f32 * accuracy.percentage();
            println!("{} first throw data for entity: {}. Adding percentage:{}", occurrences, accuracy.named_entity.name, accuracy.percentage_string());
        }else{
            println!("No first throw data for entity: {}. Adding percentage:{}", accuracy.named_entity.name, accuracy.percentage_string());
        }
    }
    println!("First throws: {:.2}%", first_percentages / first_count as f32);
    println!("General accuracy: {:.2}%", general_percentages / general_count as f32);
    (first_percentages / first_count as f32, general_percentages / general_count as f32)
}

#[cfg(test)]
mod tests{
    use crate::calc::accuracy_data::{Accuracy, EntityAccuracy, FirstThrowAccuracy};
    use crate::calc::calculation::show_difference_for_first_throws;
    use crate::team_player_data::{TEST_TEAM1, TEST_TEAM2};

    #[test]
    fn test_simple(){
        let acc_1 : Accuracy = Accuracy { named_entity : TEST_TEAM1.named_entity, throws: 4, hits:0};
        let first_1 : Accuracy = Accuracy { named_entity : TEST_TEAM1.named_entity, throws: 4, hits:0};
        let acc_2 : Accuracy = Accuracy { named_entity : TEST_TEAM2.named_entity, throws: 4, hits:4};
        let first_2 : Accuracy = Accuracy { named_entity : TEST_TEAM2.named_entity, throws: 1, hits:1};
        let firsts = FirstThrowAccuracy { accuracies: vec![first_1, first_2] };
        let accs = EntityAccuracy{accuracies: vec![acc_1, acc_2]};

        let data = show_difference_for_first_throws(&firsts, &accs);
        assert_eq!(data.0, 20.0);
        assert_eq!(data.1, 50.0);
    }
}