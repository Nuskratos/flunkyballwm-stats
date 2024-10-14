use crate::data::*;

pub const PLAYER_INVALID: TeamMember = TeamMember { name: "Invalid", id: 0 };
pub const JEROME: TeamMember = TeamMember { name: "Jerome", id: 1 };
pub const BEEF: TeamMember = TeamMember {name: "Pierre", id: 2 };
pub const SEBI: TeamMember = TeamMember { name: "Sebastian", id: 3 };
pub const FLO: TeamMember = TeamMember { name: "Florian", id: 4 };
pub const SASCHA: TeamMember = TeamMember { name: "Sascha", id: 5 };
pub const JONAS: TeamMember = TeamMember { name: "Jonas", id: 6 };
pub const LUISE: TeamMember = TeamMember { name: "Luise", id: 7 };
pub const TOBIAS: TeamMember = TeamMember { name: "Tobias", id: 8 };
pub const MALTE: TeamMember = TeamMember { name: "Malte", id: 9 };
pub const CHRIS: TeamMember = TeamMember { name: "Chris", id: 10 };
pub const HANNES: TeamMember = TeamMember { name: "Hannes", id: 11 };
pub const LAURA: TeamMember = TeamMember { name: "Laura", id: 12 };
pub const SÖNKE: TeamMember = TeamMember { name: "Sönke", id: 13 };
pub const MICHI: TeamMember = TeamMember { name: "Michi", id: 14 };
pub const SARA: TeamMember = TeamMember { name: "Sara", id: 15 };
pub const KATHI: TeamMember = TeamMember { name: "Kathi", id: 16 };
pub const TEST_PLAYER1: TeamMember = TeamMember { name: "Test1", id: 991 };
pub const TEST_PLAYER2: TeamMember = TeamMember { name: "Test2", id: 992 };
pub const TEST_PLAYER3: TeamMember = TeamMember { name: "Test3", id: 993 };
pub const TEST_PLAYER4: TeamMember = TeamMember { name: "Test4", id: 994 };
pub const TEST_PLAYER5: TeamMember = TeamMember { name: "Test5", id: 995 };
pub const TEST_PLAYER6: TeamMember = TeamMember { name: "Test6", id: 996 };
pub const TEST_PLAYER7: TeamMember = TeamMember { name: "Test7", id: 997 };
pub const TEST_PLAYER8: TeamMember = TeamMember { name: "Test8", id: 998 };


pub const TEAM_INVALID: Team = Team { name: "Illegal Team", id: 100, member_1: PLAYER_INVALID, member_2: PLAYER_INVALID };
pub const DA_HAM_SIE: Team = Team { name: "Da ham sie einfach gewonnen", id: 101, member_1: JEROME, member_2: BEEF };
pub const DOS_BROS: Team = Team { name: "Dos Bros", id: 102, member_1: SEBI, member_2: FLO };
pub const STRAMMSEIN: Team = Team { name: "Strammsein", id: 103, member_1: JONAS, member_2: SASCHA };
pub const WHITE_CLAW: Team = Team { name: "White Claw", id: 104, member_1: LUISE, member_2: TOBIAS };
pub const WEDELMEDEL: Team = Team { name: "Team Wädelmädel", id: 105, member_1: MALTE, member_2: CHRIS };
pub const GEWERTET: Team = Team { name: "Wurde das gewertet?", id: 106, member_1: HANNES, member_2: LAURA };
pub const TOBÖNKE: Team = Team { name: "Prinzessin Tobönke", id: 107, member_1: TOBIAS, member_2: SÖNKE };
pub const CHROME: Team = Team { name: "Chrome", id: 108, member_1: JEROME, member_2: CHRIS };
pub const BIERATENPARTEI: Team = Team { name: "Bieratenpartei", id: 109, member_1: SEBI, member_2: BEEF };
pub const LURCHIE: Team = Team { name: "Lurchie", id: 110, member_1: LUISE, member_2: MICHI };
pub const NAME_KOMMT_SPÄTER: Team = Team { name: "Name kommt später", id: 111, member_1: SARA, member_2: KATHI };
pub const MINDESTGRÖSSE: Team = Team { name: "#Mindestgröße", id: 112, member_1: JONAS, member_2: HANNES };
pub const TEST_TEAM1: Team = Team { name: "TestTeam1", id: 9801, member_1: TEST_PLAYER1, member_2: TEST_PLAYER2 };
pub const TEST_TEAM2: Team = Team { name: "TestTeam2", id: 9802, member_1: TEST_PLAYER3, member_2: TEST_PLAYER4 };
pub const TEST_TEAM3: Team = Team { name: "TestTeam3", id: 9803, member_1: TEST_PLAYER5, member_2: TEST_PLAYER6 };
pub const TEST_TEAM4: Team = Team { name: "TestTeam4", id: 9804, member_1: TEST_PLAYER7, member_2: TEST_PLAYER8 };

pub const NAME_WIDTH: usize = 27;

#[cfg(test)]
pub mod test {
    use crate::calc::accuracy_data::Accuracy;
    use crate::data::*;
    use crate::team_player_data::{BEEF, CHRIS, FLO, JEROME, JONAS, LUISE, MALTE, SASCHA, SEBI, TOBIAS};

    pub struct PlayerStats {
        pub member: TeamMember,
        pub accuracy: f32,
        pub drinking_speed: f32,
        pub running_round_duration: f32,
        pub strafschluck_per_game: f32,
        pub strafbeer_per_game: f32,
    }

    // Calculated from the wm2024 without gewertet. The running round duration is simply estimated and used here to check if my method for calculating the running speed is accurate
    pub const JEROME_STATS: PlayerStats = PlayerStats { member: JEROME, accuracy: 71.43, drinking_speed: 2.22, running_round_duration: 0.9, strafschluck_per_game: 0.12, strafbeer_per_game: 0.0 };
    pub const BEEF_STATS: PlayerStats = PlayerStats { member: BEEF, accuracy: 66.67, drinking_speed: 2.96, running_round_duration: 1.1, strafschluck_per_game: 0.12, strafbeer_per_game: 0.12 };
    pub const SEBI_STATS: PlayerStats = PlayerStats { member: SEBI, accuracy: 66.67, drinking_speed: 3.71, running_round_duration: 1.1, strafschluck_per_game: 0.0, strafbeer_per_game: 0.0 };
    pub const FLO_STATS: PlayerStats = PlayerStats { member: FLO, accuracy: 52.94, drinking_speed: 2.5, running_round_duration: 0.9, strafschluck_per_game: 0.25, strafbeer_per_game: 0.0 };
    pub const LUISE_STATS: PlayerStats = PlayerStats { member: LUISE, accuracy: 33.33, drinking_speed: 3.72, running_round_duration: 1.1, strafschluck_per_game: 0.33, strafbeer_per_game: 0.0 };
    pub const TOBIAS_STATS: PlayerStats = PlayerStats { member: TOBIAS, accuracy: 30.77, drinking_speed: 2.6, running_round_duration: 1.0, strafschluck_per_game: 0.0, strafbeer_per_game: 0.11 };
    pub const CHRIS_STATS: PlayerStats = PlayerStats { member: CHRIS, accuracy: 47.62, drinking_speed: 3.27, running_round_duration: 1.0, strafschluck_per_game: 0.00, strafbeer_per_game: 0.00 };
    pub const MALTE_STATS: PlayerStats = PlayerStats { member: MALTE, accuracy: 27.27, drinking_speed: 3.25, running_round_duration: 1.0, strafschluck_per_game: 0.22, strafbeer_per_game: 0.00 };
    pub const SASCHA_STATS: PlayerStats = PlayerStats { member: SASCHA, accuracy: 50.0, drinking_speed: 2.8, running_round_duration: 0.9, strafschluck_per_game: 0.12, strafbeer_per_game: 0.12 };
    pub const JONAS_STATS: PlayerStats = PlayerStats { member: JONAS, accuracy: 36.84, drinking_speed: 3.0, running_round_duration: 1.0, strafschluck_per_game: 0.38, strafbeer_per_game: 0.00 };

    pub struct TeamStats {
        pub first: PlayerStats,
        pub second: PlayerStats
    }

    pub const DA_HAM_SIE_STATS : TeamStats = TeamStats{ first:JEROME_STATS, second:BEEF_STATS};
    pub const DOS_BROS_STATS: TeamStats = TeamStats { first: FLO_STATS, second: SEBI_STATS};
    pub const STRAMMSEIN_STATS: TeamStats = TeamStats { first: SASCHA_STATS, second:JONAS_STATS };
    pub const WHITE_CLAW_STATS: TeamStats = TeamStats { first: LUISE_STATS, second: TOBIAS_STATS};
    pub const WEDELMEDEL_STATS: TeamStats = TeamStats { first: CHRIS_STATS, second: MALTE_STATS};

}
