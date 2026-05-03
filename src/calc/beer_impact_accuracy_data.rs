use crate::calc::accuracy_data::Accuracy;
use crate::data::NamedEntity;
use crate::team_player_data::AVERAGE_ENTITY;
use crate::util::{open_writer, OpenedWriter};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Clone)]
pub struct EntityBeerImpact {
    pub named_entity: NamedEntity,
    pub accuracy_points: Vec<Accuracy>,
}
impl EntityBeerImpact {
    pub fn merge(&mut self, other: &Self) {
        if self.accuracy_points.len() > other.accuracy_points.len() {
            for (i, accuracy) in other.accuracy_points.iter().enumerate() {
                self.accuracy_points[i].merge(accuracy);
            }
        } else {
            for (i, accuracy) in self.accuracy_points.iter_mut().enumerate() {
                accuracy.merge(&other.accuracy_points[i]);
            }
        }
    }
    pub fn new(entity: NamedEntity) -> EntityBeerImpact {
        EntityBeerImpact { named_entity: entity, accuracy_points: Vec::new() }
    }
    pub fn add_throw(&mut self, hit: bool, beers_drank: usize) {
        while self.accuracy_points.len() <= beers_drank {
            self.accuracy_points.push(Accuracy::new(self.named_entity.clone()));
        }
        self.accuracy_points[beers_drank].add_throw(hit);
    }

    fn documentation_string(&self) -> String {
        let mut ret_val = String::new();
        for (i, acc) in self.accuracy_points.iter().enumerate() {
            let _ = write!(ret_val, "{}: {}({:>3}/{:>3})", i, acc.percentage_string(), acc.hits, acc.throws);
        }
        ret_val
    }
}

pub struct RawBeerImpact {
    pub accuracy_at_beers_drank: Vec<Accuracy>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TournamentEntityBeerImpact {
    pub impacts: HashMap<NamedEntity, EntityBeerImpact>,
}

impl Ord for TournamentEntityBeerImpact {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.impacts.len().cmp(&other.impacts.len())
    }
}

impl PartialOrd for TournamentEntityBeerImpact {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TournamentEntityBeerImpact {
    pub fn merge(&mut self, other: &Self) {
        let mut list_of_changed: Vec<NamedEntity> = Vec::new();
        for (_, impact) in self.impacts.iter_mut() {
            let found_entity = other.impacts.iter().find(|(_, x)| x.named_entity == impact.named_entity);
            if found_entity.is_some() {
                impact.merge(found_entity.unwrap().1);
                list_of_changed.push(impact.named_entity.clone());
            }
        }
        for other_impact in other.impacts.iter() {
            if list_of_changed.contains(&other_impact.1.named_entity) {
                continue;
            }
            self.impacts.insert(other_impact.0.to_owned(), other_impact.1.to_owned());
        }
    }
    pub fn raw_points(&self) -> RawBeerImpact {
        let mut index = 0;
        let mut ret_val: RawBeerImpact = RawBeerImpact { accuracy_at_beers_drank: Vec::new() };
        loop {
            let mut changed_value = false;
            for (_, entry) in self.impacts.iter() {
                if entry.accuracy_points.len() > index {
                    while ret_val.accuracy_at_beers_drank.len() <= index {
                        ret_val.accuracy_at_beers_drank.push(Accuracy::new(AVERAGE_ENTITY.clone()));
                    }
                    ret_val.accuracy_at_beers_drank[index].merge(&entry.accuracy_points[index]);
                    changed_value = true;
                }
            }
            index = index + 1;
            if !changed_value {
                break;
            }
        }
        // divide everything by 2, as the adding counted teams & players twice
        for accuracy in &mut ret_val.accuracy_at_beers_drank {
            accuracy.hits = accuracy.hits / 2;
            accuracy.throws = accuracy.throws / 2;
        }
        ret_val
    }

    pub fn print(self) {
        println!("Possible effect of beers on accuracy for {}", "Average");
        for (i, general_values) in self.raw_points().accuracy_at_beers_drank.iter().enumerate() {
            general_values.print_for_beer_impact(i);
        }
    }
    pub fn serialize(self, file_prefix: &String, date: &String) {
        let filesuffix = "beer_impact_accuracy.csv".to_string();
        let real_writer = open_writer(date.to_string() + &filesuffix);
        self.serialize_internal(real_writer, &file_prefix);
        // Add alias writer once personal stats shall be created

        let personal_suffix = "personal_beer_impact_accuracy.csv".to_string();
        let personal_writer = open_writer(date.to_string() + &personal_suffix);
        self.serialize_internal_personal(personal_writer, false, &file_prefix);

        let alias_personal_writer = open_writer("alias".to_string()+ &date.to_string() + &personal_suffix);
        self.serialize_internal_personal(alias_personal_writer, true, &file_prefix);
    }

    fn serialize_internal(&self, mut opened_writer: OpenedWriter, file_prefix: &str) {
        if !opened_writer.file_exists {
            opened_writer.writer.write_record(&["HiddenPrefix","Getrunkene Biere", "Genauigkeit"]);
        }
        for (i, general_values) in self.raw_points().accuracy_at_beers_drank.iter().enumerate() {
            opened_writer.writer.write_record(&[file_prefix, &i.to_string(),&general_values.percentage_and_data_string()]);
        }
    }
    fn serialize_internal_personal(&self, mut opened_writer: OpenedWriter, write_alias: bool, file_prefix: &str) {
        if !opened_writer.file_exists {
            opened_writer.writer.write_record(&["HiddenPrefix", "Name", "Genauigkeit"]);
        }
        for general_values in self.impacts.iter() {
            opened_writer.writer.write_record(&[
                file_prefix,
                &general_values.1.named_entity.name_or_alias(write_alias),
                &general_values.1.documentation_string(),
            ]);
        }
    }
}
