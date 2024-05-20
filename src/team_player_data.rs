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


pub const TEAM_INVALID: Team = Team {name: "Illegal Team", id: 100, member_1:PLAYER_INVALID, member_2: PLAYER_INVALID};
pub const DA_HAM_SIE: Team = Team { name: "Da ham sie einfach gewonnen", id: 101, member_1: JEROME, member_2: BEEF };
pub const DOS_BROS: Team = Team { name: "Dos Bros", id: 102, member_1: SEBI, member_2: FLO };
pub const STRAMMSEIN: Team = Team { name: "Strammsein", id: 103, member_1: JONAS, member_2: SASCHA };
pub const WHITE_CLAW: Team = Team { name: "White Claw", id: 104, member_1: LUISE, member_2: TOBIAS };
pub const WEDELMEDEL: Team = Team { name: "Team Wädelmädel", id: 105, member_1: MALTE, member_2: CHRIS };
pub const GEWERTET: Team = Team { name: "Wurde das gewertet?", id: 106, member_1: HANNES, member_2: LAURA };

pub const NAME_WIDTH : usize = 27;
