use crate::data::AdditionalType::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AdditionalType {
    FINISHED,
    STRAFBIER,
    STRAFSCHLUCK,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TeamMember {
    pub(crate) name: &'static str,
    pub(crate) id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Team {
    pub(crate) name: &'static str,
    pub(crate) id: u32,
    pub(crate) member_1: TeamMember,
    pub(crate) member_2: TeamMember,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Result {
    pub(crate) points_left: u32,
    pub(crate) points_right: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Additional {
    pub(crate) kind: AdditionalType,
    pub(crate) source: TeamMember,
}

impl Additional {
    pub(crate) fn finish(team_member: TeamMember) -> Additional {
        Additional { kind: FINISHED, source: team_member }
    }
    fn beer(team_member: TeamMember) -> Additional {
        Additional { kind: STRAFBIER, source: team_member }
    }
    fn schluck(team_member: TeamMember) -> Additional {
        Additional { kind: STRAFSCHLUCK, source: team_member }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ARC {
    pub(crate) additional: Additional,
    pub(crate) round_nr: u32, // Round starts with 0
}

impl ARC {
    pub(crate) fn finish(team_member: &TeamMember, round_nr: u32) -> ARC {
        ARC { additional: Additional::finish(team_member.clone()), round_nr }
    }
    pub(crate) fn beer(team_member: &TeamMember, round_nr: u32) -> ARC {
        ARC { additional: Additional::beer(team_member.clone()), round_nr }
    }
    pub(crate) fn schluck(team_member: &TeamMember, round_nr: u32) -> ARC {
        ARC { additional: Additional::schluck(team_member.clone()), round_nr }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    pub(crate) thrower: TeamMember,
    pub(crate) runner: TeamMember,
    pub(crate) hit: bool,
    pub(crate) additionals: Vec<Additional>,
}

impl Round {
    pub(crate) fn additionals(&mut self, vec: Vec<Additional>) {
        self.additionals = vec
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub(crate) match_number: u32,
    pub(crate) left_team: Team,
    pub(crate) left_1: TeamMember,
    pub(crate) left_2: TeamMember,
    pub(crate) right_team: Team,
    pub(crate) right_1: TeamMember,
    pub(crate) right_2: TeamMember,
    pub(crate) result: Result,
    pub(crate) rounds: Vec<Round>,
}

impl Game {
    pub fn print(&self) {
        println!("Spielnr: {}", self.match_number);
        println!("Team: {0:<16.16} | Team: {1:<16.16}", self.left_team.name, self.right_team.name);
        println!("Spieler1: {0:<12.12} | Spieler1: {1:<12.12}", self.left_1.name, self.right_1.name);
        println!("Spieler2: {0:<12.12} | Spieler2: {1:<12.12}", self.left_2.name, self.right_2.name);
        for round in &self.rounds {
            let left1 = player_round_string(&self.left_1, &round, true);
            let left2 = player_round_string(&self.left_2, &round, true);
            let right1 = player_round_string(&self.right_1, &round, false);
            let right2 = player_round_string(&self.right_2, &round, false);
            println!("{0:^5}|{1:^5}|{2:^5}|{3:^5}|{4:^5}|{5:^5}|{6:^5}|{7:^5}", left1.0, left2.0, left1.1, left2.1, right1.0, right2.0, right1.1, right2.1);
        }
        println!("Punkte: {0:>14} | Punkte: {1:<14}", self.result.points_left, self.result.points_right);
    }
    pub fn winning_team_id(&self) -> u32 {
        if self.result.points_left > self.result.points_right {
            self.left_team.id
        } else {
            self.right_team.id
        }
    }
}

pub fn bool_vec_from_int(first_value: bool, int_vec: Vec<u32>) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    let mut curr_bool = first_value;
    for val in int_vec {
        for _i in 0..val {
            ret.push(curr_bool);
        }
        curr_bool = !curr_bool;
    }
    ret
}

pub fn player_round_string(player: &TeamMember, round: &Round, left_team: bool) -> (String, String) {
    let mut add_string: String = String::new();
    let mut round_string: String = String::new();
    for additionals in round.additionals.iter().filter(|x| x.source.id == player.id) {
        match &additionals.kind {
            FINISHED => add_string.push('\u{2713}'),
            STRAFSCHLUCK => add_string.push('S'),
            STRAFBIER => add_string.push('B')
        }
    }
    if round.runner.id == player.id {
        round_string.push('*');
    }
    if round.thrower.id == player.id {
        if round.hit {
            round_string.push('X');
        } else {
            round_string.push('/');
        }
    }
    if left_team {
        (add_string, round_string)
    } else {
        (round_string, add_string)
    }
}

pub fn results_from_additionals(additionals: &Vec<ARC>, left_team_first: bool, left_team: &Team) -> Result {
    let mut result: Result = Result { points_left: 0, points_right: 0 };
    let mut result_values: Vec<u32> = vec![7, 5, 3];
    let mut round_checker_offset = 0;
    if !left_team_first {
        round_checker_offset = 1;
    }
    for additional_round in additionals {
        if additional_round.additional.kind == FINISHED {
            if additional_round.additional.source.id == left_team.member_2.id || additional_round.additional.source.id == left_team.member_1.id {
                result.points_left = result.points_left + result_values.first().unwrap();
            } else {
                result.points_right = result.points_right + result_values.first().unwrap();
            }
            result_values.remove(0);
        }
    }
    // Adding 2 to the winning team
    if result.points_left > result.points_right {
        result.points_left = result.points_left + 2;
    } else {
        result.points_right = result.points_right + 2;
    }
    result
}

pub fn create_normalized_rounds(first_team_1: &TeamMember, first_team_2: &TeamMember, second_team_1: &TeamMember, second_team_2: &TeamMember, throw_order: Vec<bool>) -> Vec<Round> {
    let mut ret_vector = Vec::new();
    for (ix, value) in throw_order.iter().enumerate() {
        let iteration = ix % 4;
        match iteration {
            0 => ret_vector.push(Round { thrower: first_team_1.clone(), runner: second_team_1.clone(), hit: *value, additionals: vec![] }),
            1 => ret_vector.push(Round { thrower: second_team_1.clone(), runner: first_team_1.clone(), hit: *value, additionals: vec![] }),
            2 => ret_vector.push(Round { thrower: first_team_2.clone(), runner: second_team_2.clone(), hit: *value, additionals: vec![] }),
            3 => ret_vector.push(Round { thrower: second_team_2.clone(), runner: first_team_2.clone(), hit: *value, additionals: vec![] }),
            _ => {}
        }
    }
    ret_vector
}


pub fn create_normal_rounds_left_right(left_team_1: &TeamMember, left_team_2: &TeamMember, right_team_1: &TeamMember, right_team_2: &TeamMember, throw_order: Vec<bool>, add_round_info: Vec<ARC>, left_began: bool) -> Vec<Round> {
    if left_began {
        create_normal_rounds_with_additionals_and_correct_order(left_team_1, left_team_2, right_team_1, right_team_2, throw_order, add_round_info)
    } else {
        create_normal_rounds_with_additionals_and_correct_order(right_team_1, right_team_2, left_team_1, left_team_2, throw_order, add_round_info)
    }
}

pub fn create_normal_rounds_with_additionals_and_correct_order(first_team_1: &TeamMember, first_team_2: &TeamMember, second_team_1: &TeamMember, second_team_2: &TeamMember, throw_order: Vec<bool>, additional_round_info: Vec<ARC>) -> Vec<Round> {
    let mut ret_vector = Vec::new();
    let mut first_team: Vec<TeamMember> = vec![first_team_1.clone(), first_team_2.clone()];
    let mut second_team: Vec<TeamMember> = vec![second_team_1.clone(), second_team_2.clone()];
    for (ix, hit) in throw_order.iter().enumerate() {
        let mut round_additionals: Vec<Additional> = Vec::new();
        for round in &additional_round_info {
            if round.round_nr == ix as u32 { // Add additionals if from this round
                round_additionals.push(round.additional.clone());
            }
        }
        let (thrower, runner) = if ix % 2 == 0 { (first_team.first().unwrap(), second_team.first().unwrap()) } else { (second_team.first().unwrap(), first_team.first().unwrap()) };
        ret_vector.push(Round { thrower: thrower.clone(), runner: runner.clone(), hit: *hit, additionals: round_additionals });
        for round in &additional_round_info {
            if round.round_nr == ix as u32 { // Remove done drinkers. Can't be together with the loop above because otherwise the Roundcreation fails (because I'm still new at Rust)
                if round.additional.kind == FINISHED {
                    first_team.retain(|x| x.id != round.additional.source.id);
                    second_team.retain(|x| x.id != round.additional.source.id);
                }
            }
        }
        // flip members after every 2 rounds
        if (ix + 1) % 2 == 0 {
            first_team.reverse();
            second_team.reverse();
        }
    }
    ret_vector
}
