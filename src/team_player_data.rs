use crate::data::*;

pub const jerome: TeamMember = TeamMember { name: "Jerome", id: 1 };
pub const beef :TeamMember = TeamMember { name: "Pierre", id: 2 };
pub const sebi: TeamMember = TeamMember { name: "Sebastian", id: 3 };
pub const flo: TeamMember = TeamMember { name: "Florian", id: 4 };
pub const sascha: TeamMember = TeamMember { name: "Sascha", id: 5 };
pub const jonas: TeamMember = TeamMember { name: "Jonas", id: 6 };
pub const luise: TeamMember = TeamMember { name: "Luise", id: 7 };
pub const tobias: TeamMember = TeamMember { name: "Tobias", id: 8 };
pub const malte: TeamMember = TeamMember { name: "Malte", id: 9 };
pub const chris: TeamMember = TeamMember { name: "Chris", id: 10 };
pub const hannes: TeamMember = TeamMember { name: "Hannes", id: 11 };
pub const laura: TeamMember = TeamMember { name: "Laura", id: 12 };

pub const da_ham_sie: Team = Team { name: "Da ham sie einfach gewonnen", id: 1, member_1: jerome, member_2: beef };
pub const dos_bros: Team = Team { name: "Dos Bros", id: 2, member_1: sebi, member_2: flo };
pub const strammsein: Team = Team { name: "Strammsein", id: 3, member_1: jonas, member_2: sascha };
pub const white_claw: Team = Team { name: "White Claw", id: 4, member_1: luise, member_2: tobias };
pub const wedelmedel: Team = Team { name: "Team Wädelmädel", id: 5, member_1: malte, member_2: chris };
pub const gewertet: Team = Team { name:"Wurde das gewertet?", id: 6, member_1: hannes, member_2: laura };
