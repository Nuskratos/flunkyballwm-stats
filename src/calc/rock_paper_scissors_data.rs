use std::cmp::Ordering;
use std::fmt::format;
use crate::data::Entity;
use crate::team_player_data::NAME_WIDTH;
use crate::team_player_data::ENTRY_WIDTH;
use crate::util::{open_writer, OpenedWriter};

#[derive(Clone, Default)]
pub struct MatchStats {
    pub played_rounds: u32,
    pub won_rounds: u32,
}

impl MatchStats {
    pub fn estimated_win_percentage(&self) -> f32 {
        (self.won_rounds + 1) as f32 / (self.played_rounds + 2) as f32 * 100.0
    }

    pub fn win_percentage(&self) -> f32 {
        if self.played_rounds == 0 { return 0.0; }
        self.won_rounds as f32 / self.played_rounds as f32 * 100.0
    }

    pub fn string_estimated_win_percentage(&self) -> String {
        format!("{:.2}%", self.estimated_win_percentage())
    }

    pub fn string_win_percentage(&self) -> String {
        format!("{:.2}%", self.win_percentage())
    }

    pub fn add_win(&mut self) {
        self.played_rounds += 1;
        self.won_rounds += 1;
    }

    pub fn add_loss(&mut self) {
        self.played_rounds += 1;
    }
}

#[derive(Clone)]
pub struct RockPaperScissorsEnemyStats{
    pub entity: Entity,
    pub stats: MatchStats
}

pub struct RockPaperScissorSingleStats{
    pub entity: Entity,
    pub stats:MatchStats,
    pub enemies: Vec<RockPaperScissorsEnemyStats>
}
impl RockPaperScissorSingleStats{
    pub fn default(entity: &Entity) -> RockPaperScissorSingleStats{
        RockPaperScissorSingleStats{entity:entity.clone(), stats:MatchStats::default(), enemies:Vec::new()}
    }
    pub fn won(&mut self, enemy_entity: &Entity){
        self.stats.add_win();
        if let Some(enemy) = self.enemies.iter_mut().find(|e| e.entity.id() == enemy_entity.id()){
            enemy.stats.add_win();
        }else{
            let mut enemy = RockPaperScissorsEnemyStats{entity:enemy_entity.to_owned(), stats:MatchStats::default()};
            enemy.stats.add_win();
            self.enemies.push(enemy);
        }
    }
    pub fn lost(&mut self, enemy_entity: &Entity){
        self.stats.add_loss();
        if let Some(enemy) = self.enemies.iter_mut().find(|e| e.entity.id() == enemy_entity.id()){
            enemy.stats.add_loss();
        }else{
            let mut enemy = RockPaperScissorsEnemyStats{entity:enemy_entity.to_owned(), stats:MatchStats::default()};
            enemy.stats.add_loss();
            self.enemies.push(enemy);
        }
    }
    pub fn best_matchup(&self) -> RockPaperScissorsEnemyStats{
        let mut best = self.enemies.first().unwrap().to_owned();
        for enemy in &self.enemies {
            if enemy.stats.estimated_win_percentage() >= best.stats.estimated_win_percentage() {
                best = enemy.clone();
            }
        }
        best
    }
    pub fn worst_matchup(&self) -> RockPaperScissorsEnemyStats{
        let mut worst = self.enemies.first().unwrap().to_owned();
        for enemy in &self.enemies {
            if enemy.stats.estimated_win_percentage() <= worst.stats.estimated_win_percentage() {
                worst = enemy.to_owned();
            }
        }
        worst
    }
    pub fn stats_string(&self)->String{
        format!("{}({}/{})", &self.stats.string_win_percentage(), &self.stats.won_rounds, &self.stats.played_rounds)
    }
    pub fn matchup_string(write_alias:bool, enemy:&RockPaperScissorsEnemyStats) -> String{
        format!("{}({}/{}) {}",enemy.stats.string_win_percentage(), enemy.stats.won_rounds, enemy.stats.played_rounds, enemy.entity.named().name_or_alias(write_alias))
    }
    pub fn print(&self){
        let best = self.best_matchup();
        let worst = self.worst_matchup();
        println!("Team:  | {:^NAME_WIDTH$} |{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}", self.entity.named().name, self.stats.won_rounds, self.stats.played_rounds, self.stats.string_win_percentage(), self.stats.string_estimated_win_percentage());
        println!("Best:  | {:^NAME_WIDTH$} |{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}", self.best_matchup().entity.named().name, best.stats.won_rounds, best.stats.played_rounds, best.stats.string_win_percentage(), best.stats.string_estimated_win_percentage());
        println!("Worst: | {:^NAME_WIDTH$} |{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}", self.worst_matchup().entity.named().name, worst.stats.won_rounds, worst.stats.played_rounds, worst.stats.string_win_percentage(), worst.stats.string_estimated_win_percentage());
        println!("")
    }
}
impl PartialEq for RockPaperScissorSingleStats {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

// 2. Eq: Mark it as a total equality (requires PartialEq)
impl Eq for RockPaperScissorSingleStats {}

// 3. PartialOrd: Define how to compare for inequality (returns Option)
impl PartialOrd for RockPaperScissorSingleStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RockPaperScissorSingleStats {
    fn cmp(&self, other: &Self) -> Ordering {
        // Swap self and other to sort Descending
        other.stats.estimated_win_percentage()
            .partial_cmp(&self.stats.estimated_win_percentage())
            .unwrap_or(Ordering::Equal)
    }
}

pub struct RockPaperScissorStats{
    pub data : Vec<RockPaperScissorSingleStats>
}
impl RockPaperScissorStats{
     pub fn print(&self){
         println!("| {:^NAME_WIDTH$} |{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}{:^ENTRY_WIDTH$}","Name", "won", "played", "win%", "estimated");
         for entity in &self.data {
            entity.print();
         }
     }
    pub fn serialize(&self, file_prefix:&String, date: &String){
        let filesufix= "rock_paper_scissor.csv".to_string();
        let real_writer = open_writer(date.to_string()+&filesufix);
        self.serialize_internal(real_writer, false, &file_prefix);

        let alias_writer = open_writer("alias".to_string()+&date.to_string()+&filesufix);
        self.serialize_internal(alias_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, write_alias:bool, file_prefix:&String){
        if !opened_writer.file_exists{
            opened_writer.writer.write_record(&["HiddenPrefix", "Name", "Bayesian Average", "Stats", "Best Matchup", "Worst Matchup"]);
        }
        for entry in &self.data{
            opened_writer.writer.write_record(&[file_prefix, &entry.entity.named().name_or_alias(write_alias), &entry.stats.string_estimated_win_percentage(), &entry.stats_string(), &RockPaperScissorSingleStats::matchup_string(write_alias, &entry.best_matchup()),&RockPaperScissorSingleStats::matchup_string(write_alias, &entry.worst_matchup())]);
        }
    }
 }