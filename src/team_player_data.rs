use crate::data::*;

pub const PLAYER_INVALID: TeamMember = TeamMember {name: "Invalid", id: 0};
pub const JEROME: TeamMember = TeamMember { name: "Jerome", id: 1 };
pub const BEEF: TeamMember = TeamMember { name: "Pierre", id: 2 };
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
pub const TEST_PLAYER1: TeamMember = TeamMember {name: "Test1", id: 991};
pub const TEST_PLAYER2: TeamMember = TeamMember {name: "Test2", id: 992};
pub const TEST_PLAYER3: TeamMember = TeamMember {name: "Test3", id: 993};
pub const TEST_PLAYER4: TeamMember = TeamMember {name: "Test4", id: 994};
pub const TEST_PLAYER5: TeamMember = TeamMember {name: "Test5", id: 995};
pub const TEST_PLAYER6: TeamMember = TeamMember {name: "Test6", id: 996};


pub const TEAM_INVALID: Team = Team {name: "Illegal Team", id: 100, member_1:PLAYER_INVALID, member_2: PLAYER_INVALID};
pub const DA_HAM_SIE: Team = Team { name: "Da ham sie einfach gewonnen", id: 101, member_1: JEROME, member_2: BEEF };
pub const DOS_BROS: Team = Team { name: "Dos Bros", id: 102, member_1: SEBI, member_2: FLO };
pub const STRAMMSEIN: Team = Team { name: "Strammsein", id: 103, member_1: JONAS, member_2: SASCHA };
pub const WHITE_CLAW: Team = Team { name: "White Claw", id: 104, member_1: LUISE, member_2: TOBIAS };
pub const WEDELMEDEL: Team = Team { name: "Team Wädelmädel", id: 105, member_1: MALTE, member_2: CHRIS };
pub const GEWERTET: Team = Team { name: "Wurde das gewertet?", id: 106, member_1: HANNES, member_2: LAURA };
pub const TEST_TEAM1: Team = Team { name: "TestTeam1", id: 9801, member_1: TEST_PLAYER1, member_2: TEST_PLAYER2 };
pub const TEST_TEAM2: Team = Team { name: "TestTeam2", id: 9802, member_1: TEST_PLAYER3, member_2: TEST_PLAYER4 };
pub const TEST_TEAM3: Team = Team { name: "TestTeam3", id: 9803, member_1: TEST_PLAYER5, member_2: TEST_PLAYER6 };

pub const NAME_WIDTH : usize = 27;
