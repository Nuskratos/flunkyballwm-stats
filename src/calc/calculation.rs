use std::collections::HashMap;
use std::{error::Error, io, process};
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::Writer;
use crate::calc::accuracy_data::{Accuracy, EntityAccuracy};
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