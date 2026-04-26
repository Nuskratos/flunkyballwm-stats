use crate::data::AdditionalType::*;
pub const GOTHIC_CHARACTERS: [&str; 100] = [
    "Diego",
    "Xardas",
    "Milten",
    "Gorn",
    "Lester",
    "Lee",
    "Mud",
    "Cor Angar",
    "Thorus",
    "Lares",
    "Gomez",
    "Saturas",
    "Cor Kalom",
    "Y'Berion",
    "Raven",
    "Riordian",
    "Cronos",
    "Myxir",
    "Nefarius",
    "Corristo",
    "Wolf",
    "Torlof",
    "Cipher",
    "Scar",
    "Arto",
    "Bartholo",
    "Bullit",
    "Bloodwyn",
    "Jackal",
    "Fletcher",
    "Cavalorn",
    "Scatty",
    "Kirgo",
    "Kharim",
    "Gor Na Toth",
    "Gor Na Bar",
    "Gor Na Drak",
    "Cord",
    "Jarvis",
    "Mordrag",
    "Baal Lukor",
    "Baal Oran",
    "Baal Tami",
    "Baal Namib",
    "Baal Tyon",
    "Baal Kagan",
    "Baal Isidro",
    "Baal Parvez",
    "Fortuno",
    "Joru",
    "Harlok",
    "Shrat",
    "Melvin",
    "Dusty",
    "Gor Na Ran",
    "Fisk",
    "Dexter",
    "Sly",
    "Fingers",
    "Whistler",
    "Huno",
    "Graham",
    "Snarf",
    "Ratford",
    "Drax",
    "Nek",
    "Kirgo",
    "Cutter",
    "Stone",
    "Skip",
    "Sharky",
    "Roscoe",
    "Buster",
    "Wedge",
    "Butch",
    "Silas",
    "Swiney",
    "Okyl",
    "Baloro",
    "Pacho",
    "Grim",
    "Herek",
    "Jesse",
    "Kyle",
    "Gilbert",
    "Damarok",
    "Drago",
    "Rodriguez",
    "Torrez",
    "Scorpio",
    "Cavalorn",
    "Aidan",
    "Santino",
    "Alberto",
    "Quentin",
    "Inextremo",
    "Ur-Shak",
    "Tarrok",
    "Aleph",
    "Glen",
];

pub const POE_LEAGUES: [&str; 47] = [
    "Onslaught",   // 0.11.0
    "Anarchy",     // 0.11.0
    "Domination",  // 1.0.0
    "Nemesis",     // 1.0.0
    "Invasion",    // 1.1.0
    "Ambush",      // 1.1.0
    "Beyond",      // 1.2.0
    "Rampage",     // 1.2.0
    "Bloodlines",  // 1.3.0
    "Torment",     // 1.3.0
    "Tempest",     // 2.0.0
    "Warbands",    // 2.0.0
    "Talisman",    // 2.1.0
    "Perandus",    // 2.2.0
    "Prophecy",    // 2.3.0
    "Essence",     // 2.4.0
    "Breach",      // 2.5.0
    "Legacy",      // 2.6.0
    "Harbinger",   // 3.0.0
    "Abyss",       // 3.1.0
    "Bestiary",    // 3.2.0
    "Incursion",   // 3.3.0
    "Delve",       // 3.4.0
    "Betrayal",    // 3.5.0
    "Synthesis",   // 3.6.0
    "Legion",      // 3.7.0
    "Blight",      // 3.8.0
    "Metamorph",   // 3.9.0
    "Delirium",    // 3.10.0
    "Harvest",     // 3.11.0
    "Heist",       // 3.12.0
    "Ritual",      // 3.13.0
    "Ultimatum",   // 3.14.0
    "Expedition",  // 3.15.0
    "Scourge",     // 3.16.0
    "Archnemesis", // 3.17.0
    "Sentinel",    // 3.18.0
    "Kalandra",    // 3.19.0
    "Sanctum",     // 3.20.0
    "Crucible",    // 3.21.0
    "Ancestor",    // 3.22.0
    "Affliction",  // 3.23.0
    "Necropolis",  // 3.24.0
    "Settlers",    // 3.25.0
    "Mercenaries", // 3.26.0
    "Keepers",     // 3.27.0
    "Mirage",      // 3.28.0 (Aktuell)
];


#[derive(Clone, Debug, PartialEq)]
pub enum AdditionalType {
    FINISHED,
    STRAFBIER,
    STRAFSCHLUCK,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
pub struct NamedEntity {
    pub(crate) name :&'static str,
    pub(crate) alias: &'static str,
    pub(crate) id: u32
}
impl NamedEntity{
    pub fn name_or_alias(&self, write_alias:bool)-> String{
        if write_alias{
            self.alias.to_string()
        }else{
            self.name.to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
pub struct TeamMember {
    pub(crate) named_entity: NamedEntity,
}

impl TeamMember{
    pub const fn create(name: &'static str, id: u32)->TeamMember{
        let mut alias = name;
        if id < GOTHIC_CHARACTERS.len() as u32{
            alias = GOTHIC_CHARACTERS[id as usize];
        }
        TeamMember{named_entity:NamedEntity{name,alias,id }}
    }
    pub fn name(&self) -> &str{
        self.named_entity.name
    }
    pub fn alias(&self) -> &str{
        self.named_entity.alias
    }
    pub fn id(&self) -> u32{
        self.named_entity.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Team {
    pub(crate) named_entity: NamedEntity,
    pub(crate) member_1: TeamMember,
    pub(crate) member_2: TeamMember,
}
impl Team{
    pub const fn create(name:&'static str, id:u32, member1: TeamMember, member2:TeamMember) -> Team{
        let mut alias = name;
        if id-1000 < POE_LEAGUES.len() as u32{
            alias = POE_LEAGUES[(id-1000) as usize];
        }
        Team{named_entity:NamedEntity{name, alias, id}, member_1:member1, member_2:member2}
    }
    pub fn name(&self) -> &str{
        self.named_entity.name
    }
    pub fn alias(&self) -> &str{
        self.named_entity.alias
    }
    pub fn id(&self) -> u32{
        self.named_entity.id
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Entity{
    Team(Team), Player(TeamMember)
}
impl Entity{
    pub fn named(&self) -> &NamedEntity{
        match self {
            Entity::Team(t) => { &t.named_entity},
            Entity::Player(m) => &m.named_entity
        }
    }
    pub fn id(&self)->u32{
        self.named().id
    }
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
    pub(crate) special_first_throw: Option<Round>
}

impl Game {
    pub fn first_throw(&self) -> &Round {
        self.special_first_throw
            .as_ref()
            .or_else(|| self.rounds.first())
            .expect("game must contain either a special first throw or at least one round")
    }
    pub fn additionals_vec(&self) -> Vec<ARC>{
        let mut ret :Vec<ARC> = Vec::new();
        for (ix,round) in self.rounds.iter().enumerate() {
            for add in round.additionals.iter(){
                ret.push(ARC{additional: add.clone(), round_nr: ix as u32});
            }
        }
        ret
    }
    pub fn print(&self) {
        println!("Spielnr: {}", self.match_number);
        println!("Team: {0:<16.16} | Team: {1:<16.16}", self.left_team.named_entity.name, self.right_team.named_entity.name);
        println!("Spieler1: {0:<12.12} | Spieler1: {1:<12.12}", self.left_1.named_entity.name, self.right_1.named_entity.name);
        println!("Spieler2: {0:<12.12} | Spieler2: {1:<12.12}", self.left_2.named_entity.name, self.right_2.named_entity.name);
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
            self.left_team.named_entity.id
        } else {
            self.right_team.named_entity.id
        }
    }
    pub fn winning_team(&self)->Team{
        if self.result.points_left > self.result.points_right{
            self.left_team.to_owned()
        }else{
            self.right_team.to_owned()
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
    for additionals in round.additionals.iter().filter(|x| x.source.named_entity.id == player.id()) {
        match &additionals.kind {
            FINISHED => add_string.push('\u{2713}'),
            STRAFSCHLUCK => add_string.push('S'),
            STRAFBIER => add_string.push('B')
        }
    }
    if round.runner.named_entity.id == player.named_entity.id {
        round_string.push('*');
    }
    if round.thrower.named_entity.id == player.named_entity.id {
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

pub fn results_from_additionals(additionals: &Vec<ARC>, left_team: &Team) -> Result {
    let mut result: Result = Result { points_left: 0, points_right: 0 };
    let mut result_values: Vec<u32> = vec![7, 5, 3];
    for additional_round in additionals {
        if additional_round.additional.kind == FINISHED {
            if additional_round.additional.source.named_entity.id == left_team.member_2.named_entity.id || additional_round.additional.source.named_entity.id == left_team.member_1.named_entity.id {
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
                    first_team.retain(|x| x.named_entity.id != round.additional.source.id());
                    second_team.retain(|x| x.named_entity.id != round.additional.source.id());
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
