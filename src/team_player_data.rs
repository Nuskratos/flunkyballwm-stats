use crate::data::*;

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

pub const DA_HAM_SIE: Team = Team { name: "Da ham sie einfach gewonnen", id: 1, member_1: JEROME, member_2: BEEF };
pub const DOS_BROS: Team = Team { name: "Dos Bros", id: 2, member_1: SEBI, member_2: FLO };
pub const STRAMMSEIN: Team = Team { name: "Strammsein", id: 3, member_1: JONAS, member_2: SASCHA };
pub const WHITE_CLAW: Team = Team { name: "White Claw", id: 4, member_1: LUISE, member_2: TOBIAS };
pub const WEDELMEDEL: Team = Team { name: "Team Wädelmädel", id: 5, member_1: MALTE, member_2: CHRIS };
pub const GEWERTET: Team = Team { name: "Wurde das gewertet?", id: 6, member_1: HANNES, member_2: LAURA };