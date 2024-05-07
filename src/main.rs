use AdditionalType::{FINISHED, STRAFBIER, STRAFSCHLUCK};

fn main() {
    //creating baselines
    let jerome = TeamMember { name: String::from("Jerome"), id: 1 };
    let beef = TeamMember { name: String::from("Pierre"), id: 2 };
    let sebi = TeamMember { name: String::from("Sebastian"), id: 3 };
    let flo = TeamMember { name: String::from("Florian"), id: 4 };
    let sascha = TeamMember { name: String::from("Sascha"), id: 5 };
    let jonas = TeamMember { name: String::from("Jonas"), id: 6 };
    let luise = TeamMember { name: String::from("Luise"), id: 7 };
    let tobias = TeamMember { name: String::from("Tobias"), id: 8 };
    let malte = TeamMember { name: String::from("Malte"), id: 9 };
    let chris = TeamMember { name: String::from("Chris"), id: 10 };
    let hannes = TeamMember { name: String::from("Hannes"), id: 11 };
    let laura = TeamMember { name: String::from("Laura"), id: 12 };

    let da_ham_sie = Team { name: String::from("Da ham sie einfach gewonnen"), id: 1, member_1: jerome.clone(), member_2: beef.clone() };
    let dos_bros = Team { name: String::from("Dos Bros"), id: 2, member_1: sebi.clone(), member_2: flo.clone() };
    let strammsein = Team { name: String::from("Strammsein"), id: 3, member_1: jonas.clone(), member_2: sascha.clone() };
    let white_claw = Team { name: String::from("White Claw"), id: 4, member_1: luise.clone(), member_2: tobias.clone() };
    let wedelmedel = Team { name: String::from("Team Wädelmädel"), id: 5, member_1: malte.clone(), member_2: chris.clone() };
    let gewertet = Team { name: String::from("Wurde das gewertet?"), id: 6, member_1: hannes.clone(), member_2: laura.clone() };

    //let game1 = first_game_old(strammsein.clone(), sascha.clone(), jonas.clone(), white_claw.clone(), tobias.clone(), luise.clone());
    //let game2 = second_game_old_faulty(gewertet.clone(), laura.clone(), hannes.clone(), dos_bros.clone(), flo.clone(), sebi.clone());
    let game1_new = first_game_new(strammsein.clone(), sascha.clone(), jonas.clone(), white_claw.clone(), tobias.clone(), luise.clone());
    let game2_new = second_game_new(gewertet.clone(), laura.clone(), hannes.clone(), dos_bros.clone(), flo.clone(), sebi.clone());
    let game3 = game_3(da_ham_sie.clone(), jerome.clone(), beef.clone(), wedelmedel.clone(), chris.clone(), malte.clone());
    let game4 = game_4(dos_bros.clone(), flo.clone(), sebi.clone(), da_ham_sie.clone(), jerome.clone(), beef.clone());
    let game5 = game_5(wedelmedel.clone(), malte.clone(), chris.clone(), strammsein.clone(), jonas.clone(), sascha.clone());

    game5.print();

    //println!("{:?}", game5);

    /*if(game2 == game2_new){
        println!("THey the same")
    }else{
        println!("{:?}", game2);
        println!("{:?}", game2_new);
    }*/
}

fn create_normalized_rounds(first_team_1: TeamMember, first_team_2: TeamMember, second_team_1: TeamMember, second_team_2: TeamMember, throw_order: Vec<bool>) -> Vec<Round> {
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

fn create_normal_rounds_left_right(left_team_1: TeamMember, left_team_2: TeamMember, right_team_1: TeamMember, right_team_2: TeamMember, throw_order: Vec<bool>, add_round_info: Vec<AdditionalRoundCombination>, left_began: bool) -> Vec<Round> {
    if left_began {
        create_normal_rounds_with_additionals_and_correct_order(left_team_1, left_team_2, right_team_1, right_team_2, throw_order, add_round_info)
    } else {
        create_normal_rounds_with_additionals_and_correct_order(right_team_1, right_team_2, left_team_1, left_team_2, throw_order, add_round_info)
    }
}

fn create_normal_rounds_with_additionals_and_correct_order(first_team_1: TeamMember, first_team_2: TeamMember, second_team_1: TeamMember, second_team_2: TeamMember, throw_order: Vec<bool>, additional_round_info: Vec<AdditionalRoundCombination>) -> Vec<Round> {
    let mut ret_vector = Vec::new();
    let mut first_team: Vec<TeamMember> = vec![first_team_1, first_team_2];
    let mut second_team: Vec<TeamMember> = vec![second_team_1, second_team_2];
    for (ix, hit) in throw_order.iter().enumerate() {
        let mut round_additionals: Vec<Additional> = Vec::new();
        for round in &additional_round_info {
            if round.round_nr == ix as u32 { // Add additionals if from this round
                round_additionals.push(round.additional.clone());
            }
        }
        if ix % 2 == 0 { // Add the Round information
            ret_vector.push(Round { thrower: first_team.first().unwrap().clone(), runner: second_team.first().unwrap().clone(), hit: *hit, additionals: round_additionals });
        } else {
            ret_vector.push(Round { thrower: second_team.first().unwrap().clone(), runner: first_team.first().unwrap().clone(), hit: *hit, additionals: round_additionals });
        }
        for round in &additional_round_info {
            if round.round_nr == ix as u32 { // Remove done drinkers. Can't be together with the loop above because otherwise the Roundcreation fails (because I'm still new at Rust)
                if (round.additional.kind == FINISHED) {
                    first_team.retain(|x| x.id != round.additional.source.id);
                    second_team.retain(|x| x.id != round.additional.source.id);
                }
            }
        }
        // flip members after every 2 rounds
        if ((ix + 1) % 2 == 0) {
            first_team.reverse();
            second_team.reverse();
        }
    }
    ret_vector
}

fn results_from_additionals(additionals: &Vec<AdditionalRoundCombination>, left_team_first: bool) -> Result {
    let mut result: Result = Result { points_left: 0, points_right: 0 };
    let mut result_values: Vec<u32> = vec![7, 5, 3];
    let mut round_checker_offset = 0;
    if (!left_team_first) {
        round_checker_offset = 1;
    }
    for  additional_round in additionals {
        if (additional_round.additional.kind == FINISHED) {
            if ((additional_round.round_nr + round_checker_offset) % 2 == 0) {
                result.points_left = result.points_left + result_values.first().unwrap();
                result_values.remove(0);
            } else {
                result.points_right = result.points_right + result_values.first().unwrap();
                result_values.remove(0);
            }
        }
    }
    // Adding 2 to the winning team
    if (result.points_left > result.points_right) {
        result.points_left = result.points_left + 2;
    } else {
        result.points_right = result.points_right + 2;
    }
    result
}

fn first_game_new(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals: Vec<AdditionalRoundCombination> = vec![AdditionalRoundCombination::schluck(left_1.clone(), 0), AdditionalRoundCombination::finish(left_1.clone(), 14), AdditionalRoundCombination::finish(left_2.clone(), 14)];
    let result = results_from_additionals(&additionals, left_began);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true], additionals.clone(), left_began);
    Game { result, match_number: 1, left_team, right_team, rounds }
}

fn second_game_new(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let mut additionals = vec![AdditionalRoundCombination::finish(right_1.clone(), 4), AdditionalRoundCombination::finish(right_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began);
    let rounds = create_normal_rounds_left_right(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(),
                                                 vec![true, true, true, true, false, true], additionals.clone(), left_began);
    Game { result, match_number: 2, left_team, right_team, rounds }
}

fn game_3(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![AdditionalRoundCombination::finish(left_1.clone(), 6), AdditionalRoundCombination::finish(left_2.clone(), 6)];
    let result = results_from_additionals(&additionals, left_began);
    let rounds = create_normal_rounds_left_right(left_1, left_2, right_1, right_2, vec![true, true, true, false, false, true, true], additionals, left_began);
    Game { result, match_number: 3, left_team, right_team, rounds }
}
fn game_4(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = true;
    let additionals = vec![AdditionalRoundCombination::finish(left_1.clone(), 6), AdditionalRoundCombination::finish(right_1.clone(), 7), AdditionalRoundCombination::finish(left_2.clone(), 8)];
    let result = results_from_additionals(&additionals, left_began);
    let rounds = create_normal_rounds_left_right(left_1, left_2, right_1, right_2, vec![false, false, true, true, true, false, true, true, true], additionals, left_began);
    Game{result, match_number:4, left_team, right_team, rounds}
}
fn game_5(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let left_began = false;
    let additionals = vec![AdditionalRoundCombination::schluck(right_1.clone(),4), AdditionalRoundCombination::finish(right_1.clone(), 10), AdditionalRoundCombination::finish(right_2.clone(),10)];
    let result = results_from_additionals(&additionals, left_began);
    let rounds = create_normal_rounds_left_right(left_1, left_2, right_1, right_2, vec![true, false, false, true, false, false, true,false,false,true,true], additionals, left_began);
    Game {result, match_number:5, left_team, right_team, rounds}
}

#[derive(Clone, Debug, PartialEq)]
enum AdditionalType {
    FINISHED,
    STRAFBIER,
    STRAFSCHLUCK,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TeamMember {
    name: String,
    id: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Team {
    name: String,
    id: u32,
    member_1: TeamMember,
    member_2: TeamMember,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Result {
    points_left: u32,
    points_right: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Additional {
    kind: AdditionalType,
    source: TeamMember,
}

impl Additional {
    fn finish(team_member: TeamMember) -> Additional {
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
pub struct AdditionalRoundCombination {
    additional: Additional,
    round_nr: u32, // Round starts with 0
}

impl AdditionalRoundCombination {
    fn finish(team_member: TeamMember, round_nr: u32) -> AdditionalRoundCombination {
        AdditionalRoundCombination { additional: Additional::finish(team_member), round_nr }
    }
    fn beer(team_member: TeamMember, round_nr: u32) -> AdditionalRoundCombination {
        AdditionalRoundCombination { additional: Additional::beer(team_member), round_nr }
    }
    fn schluck(team_member: TeamMember, round_nr: u32) -> AdditionalRoundCombination {
        AdditionalRoundCombination { additional: Additional::schluck(team_member), round_nr }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    thrower: TeamMember,
    runner: TeamMember,
    hit: bool,
    additionals: Vec<Additional>,
}

impl Round {
    fn additionals(&mut self, vec: Vec<Additional>) {
        self.additionals = vec
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    match_number: u32,
    left_team: Team,
    right_team: Team,
    result: Result,
    rounds: Vec<Round>,
}
fn player_round_string(player : &TeamMember, round:  &Round, left_team : bool) -> (String, String){
    let mut add_string : String = String::new();
    let mut round_string : String = String::new();
    for additionals in round.additionals.iter().filter(|x| x.source.id == player.id){
        match additionals.kind{
            FINISHED=>add_string.push('v'),
            STRAFSCHLUCK=>add_string.push('S'),
            STRAFBIER=>add_string.push('B')
        }
    }
    if(round.runner.id == player.id){
        round_string.push('*');
    }
    if(round.thrower.id == player.id){
        if(round.hit){
            round_string.push('X');
        }else{
            round_string.push('/');
        }
    }
    if(left_team){
        (add_string, round_string)
    }else{
        (round_string, add_string)
    }
}

impl Game{
    fn print(&self){
        println!("Spielnr: {}", self.match_number);
        println!("Team: {0:<16.16} | Team: {1:<16.16}", self.left_team.name, self.right_team.name);
        println!("Spieler1: {0:<12.12} | Spieler1: {1:<12.12}", self.left_team.member_1.name, self.right_team.member_1.name);
        println!("Spieler2: {0:<12.12} | Spieler2: {1:<12.12}", self.left_team.member_2.name, self.right_team.member_2.name);
        for round in &self.rounds{
            let left1 = player_round_string(&self.left_team.member_1, &round, true);
            let left2 = player_round_string(&self.left_team.member_2, &round, true);
            let right1 = player_round_string(&self.right_team.member_1, &round, false);
            let right2 = player_round_string(&self.right_team.member_2, &round, false);
            println!("{0:^5}|{1:^5}|{2:^5}|{3:^5}|{4:^5}|{5:^5}|{6:^5}|{7:^5}", left1.0, left2.0, left1.1, left2.1, right1.0, right2.0, right1.1, right2.1);
        }
        println!("Punkte: {0:>14} | Punkte: {1:<14}", self.result.points_left, self.result.points_right );
    }
}

fn first_game_old(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let mut rounds = create_normalized_rounds(left_1.clone(), left_2.clone(), right_1.clone(), right_2.clone(), vec![true, false, false, false, true, false, false, false, false, false, false, true, false, false, true]);
    let straf_sasch = Additional { kind: STRAFSCHLUCK, source: left_1.clone() };
    rounds.first_mut().unwrap().additionals(vec![straf_sasch]);

    let finished_sasch = Additional { kind: FINISHED, source: left_1.clone() };
    let finished_jonas = Additional { kind: FINISHED, source: left_2.clone() };
    rounds.last_mut().unwrap().additionals(vec![finished_sasch, finished_jonas]);
    let result = Result { points_left: 14, points_right: 0 };
    Game { result, match_number: 1, left_team, right_team, rounds }
}

//THIS IS FAULTY, because Florian was finished but according to this sheet ran in the 2nd to last throw
fn second_game_old_faulty(left_team: Team, left_1: TeamMember, left_2: TeamMember, right_team: Team, right_1: TeamMember, right_2: TeamMember) -> Game {
    let result = Result { points_left: 0, points_right: 14 };
    let mut rounds = create_normalized_rounds(right_1.clone(), right_2.clone(), left_1.clone(), left_2.clone(), vec![true, true, true, true, true, false, true]); //
    let finish_flo = Additional { kind: FINISHED, source: right_1.clone() };
    let finish_sebi = Additional::finish(right_2.clone());
    rounds.get_mut(4).unwrap().additionals(vec![finish_flo]);
    rounds.last_mut().unwrap().additionals(vec![finish_sebi]);
    Game { result, match_number: 2, left_team, right_team, rounds }
}