use std::borrow::ToOwned;
use crate::data::*;
pub const PLAYER_INVALID: TeamMember = TeamMember {
    named_entity: NamedEntity {
        name: "Invalid",
        alias: "Invalid",
        id: 0,
    },
};
pub const JEROME: TeamMember = TeamMember::create("Jerome", 1);
pub const BEEF: TeamMember = TeamMember::create("Pierre", 2);
pub const SEBI: TeamMember = TeamMember::create("Sebastian", 3);
pub const FLO: TeamMember = TeamMember::create("Florian", 4);
pub const SASCHA: TeamMember = TeamMember::create("Sascha", 5);
pub const JONAS: TeamMember = TeamMember::create("Jonas", 6);
pub const LUISE: TeamMember = TeamMember::create("Luise", 7);
pub const TOBIAS: TeamMember = TeamMember::create("Tobias", 8);
pub const MALTE: TeamMember = TeamMember::create("Malte", 9);
pub const CHRIS: TeamMember = TeamMember::create("Chris", 10);
pub const HANNES: TeamMember = TeamMember::create("Hannes", 11);
pub const LAURA: TeamMember = TeamMember::create("Laura", 12);
pub const SÖNKE: TeamMember = TeamMember::create("Sönke", 13);
pub const MICHI: TeamMember = TeamMember::create("Michi", 14);
pub const SARA: TeamMember = TeamMember::create("Sara", 15);
pub const KATHI: TeamMember = TeamMember::create("Kathi", 16);
pub const CHRISTOPH: TeamMember = TeamMember::create("Christoph", 17);
pub const JACKY: TeamMember = TeamMember::create("Jacky", 18);
pub const PATRICK: TeamMember = TeamMember::create("Patrick", 19);
pub const MARCEL: TeamMember = TeamMember::create("Marcel", 20);
pub const JAN_W: TeamMember = TeamMember::create("Jan W.", 21);
pub const LENA: TeamMember = TeamMember::create("Lena", 22);
pub const JAKOB_HELLWACH: TeamMember = TeamMember::create("Jakob Hellwach", 23);
pub const NIKO: TeamMember = TeamMember::create("Niko", 24);
pub const DAVID: TeamMember = TeamMember::create("David", 25);
pub const ARON: TeamMember = TeamMember::create("Aron", 26);
pub const JESSI: TeamMember = TeamMember::create("Jessi", 27);
pub const MAXI: TeamMember = TeamMember::create("Maxi", 28);
pub const JAKOB_ALTBIER: TeamMember = TeamMember::create("Jakob Altbier", 29);
pub const DZANA : TeamMember = TeamMember::create("Dzana", 30);
pub const VERENA: TeamMember = TeamMember::create("Verena", 31);//TODO Name of Team Bride member
pub const KATI_JGA: TeamMember = TeamMember::create("Kati", 32);
pub const SOPHIE: TeamMember = TeamMember::create("Sophie", 33);
pub const JOELINA: TeamMember = TeamMember::create("Joelina", 34);
pub const SARAH_BENKE: TeamMember = TeamMember::create("Sarah B.", 35);
pub const MICHELLE: TeamMember = TeamMember::create("Michelle", 36);
pub const MARIAM: TeamMember = TeamMember::create("Mariam", 37);
pub const FLO_TIMMY: TeamMember = TeamMember::create("Florian Timmy", 38);
pub const ARNE: TeamMember = TeamMember::create("Arne", 39);
pub const RENE: TeamMember = TeamMember::create("Rene", 40);
pub const LYNN: TeamMember = TeamMember::create("Lynn", 41);
pub const LISA: TeamMember = TeamMember::create("Lisa", 42);
pub const XATUN: TeamMember = TeamMember::create("Xatun", 43);
pub const JACQUELINE: TeamMember = TeamMember::create("Jacqueline", 44);

pub const TEST_PLAYER1: TeamMember = TeamMember::create("Test1", 901);
pub const TEST_PLAYER2: TeamMember = TeamMember::create("Test2", 902);
pub const TEST_PLAYER3: TeamMember = TeamMember::create("Test3", 903);
pub const TEST_PLAYER4: TeamMember = TeamMember::create("Test4", 904);
pub const TEST_PLAYER5: TeamMember = TeamMember::create("Test5", 905);
pub const TEST_PLAYER6: TeamMember = TeamMember::create("Test6", 906);
pub const TEST_PLAYER7: TeamMember = TeamMember::create("Test7", 907);
pub const TEST_PLAYER8: TeamMember = TeamMember::create("Test8", 908);
pub const AVERAGE_ENTITY: NamedEntity = NamedEntity{name:"Average", alias:"Average", id:999};
pub const AVERAGE_PLAYER: TeamMember = TeamMember{named_entity:AVERAGE_ENTITY};


pub const TEAM_INVALID: Team = Team::create("Illegal Team", 1000, PLAYER_INVALID, PLAYER_INVALID);
pub const DA_HAM_SIE: Team = Team::create("Da ham sie einfach gewonnen", 1001, JEROME, BEEF);
pub const DOS_BROS: Team = Team::create("Dos Bros", 1002, SEBI, FLO);
pub const STRAMMSEIN: Team = Team::create("Strammsein", 1003, JONAS, SASCHA);
pub const WHITE_CLAW: Team = Team::create("White Claw", 1004, LUISE, TOBIAS);
pub const WEDELMEDEL: Team = Team::create("Team Wädelmädel", 1005, MALTE, CHRIS);
pub const GEWERTET: Team = Team::create("Wurde das gewertet?", 1006, HANNES, LAURA);
pub const TOBÖNKE: Team = Team::create("Prinzessin Tobönke", 1007, TOBIAS, SÖNKE);
pub const CHROME: Team = Team::create("Chrome", 1008, JEROME, CHRIS);
pub const BIERATENPARTEI: Team = Team::create("Bieratenpartei", 1009, SEBI, BEEF);
pub const LURCHIE: Team = Team::create("Lurchie", 1010, LUISE, MICHI);
pub const NAME_KOMMT_SPÄTER: Team = Team::create("Name kommt später", 1011, SARA, KATHI);
pub const MINDESTGRÖSSE: Team = Team::create("#Mindestgröße", 1012, JONAS, HANNES);
pub const MC_FLANKY: Team = Team::create("Mc Flankygeil", 1013, JACKY, MARCEL);
pub const FLANKY_KRIEG: Team = Team::create("Flankyball ist Krieg", 1014, PATRICK, CHRISTOPH);
pub const RESERVISTEN: Team = Team::create("Reservisten", 1015, TOBIAS, JAN_W);
pub const PAEDAGOGISCH: Team = Team::create("Pädagogisch wertvoll", 1016, LUISE, LENA);
pub const HELLWACH: Team = Team::create("Team Hellwach", 1017, JAKOB_HELLWACH, NIKO);
pub const JGA2:Team = Team::create("JGA2", 1018, KATI_JGA, SOPHIE);
pub const COON_AND_FRIENDS:Team = Team::create("Coon & Friends", 1019, CHRIS, DAVID);
pub const JESS_RON:Team = Team::create("JessRon", 1020, ARON, JESSI);
pub const ALTBIERBURSCHEN:Team=Team::create("Altbierburschen", 1021, MAXI, JAKOB_ALTBIER);
pub const TEAM_BRIDE:Team=Team::create("Team Bride", 1022, DZANA, VERENA);
pub const JBG:Team=Team::create("JBG", 1023, JOELINA, SARAH_BENKE);
pub const JGA1:Team=Team::create("JGA1", 1024, MARIAM, MICHELLE);
pub const TIMMY:Team=Team::create("Team Timmy sprengen", 1025, FLO_TIMMY, ARNE);
pub const PEGELPROFIS:Team=Team::create("Die Pegelprofis", 1026, RENE, LYNN);
pub const JGA3:Team=Team::create("JGA3", 1027, LISA, XATUN);
pub const MR_MRS_SMITH:Team=Team::create("Mr. & Mrs. Smith", 1028, JACQUELINE, JEROME);
pub const TEST_TEAM1: Team = Team::create("TestTeam1", 9801, TEST_PLAYER1, TEST_PLAYER2);
pub const TEST_TEAM2: Team = Team::create("TestTeam2", 9802, TEST_PLAYER3, TEST_PLAYER4);
pub const TEST_TEAM3: Team = Team::create("TestTeam3", 9803, TEST_PLAYER5, TEST_PLAYER6);
pub const TEST_TEAM4: Team = Team::create("TestTeam4", 9804, TEST_PLAYER7, TEST_PLAYER8);
pub const AVERAGE_TEAM: Team = Team{named_entity:AVERAGE_ENTITY, member_1:AVERAGE_PLAYER, member_2:AVERAGE_PLAYER};

pub const NAME_WIDTH: usize = 27;
pub const ENTRY_WIDTH: usize = 10;

#[cfg(test)]
pub mod test {
    use crate::calc::accuracy_data::Accuracy;
    use crate::data::*;
    use crate::team_player_data::{
        BEEF, CHRIS, FLO, JEROME, JONAS, LUISE, MALTE, SASCHA, SEBI, TOBIAS,
    };

    pub struct PlayerStats {
        pub member: TeamMember,
        pub accuracy: f32,
        pub drinking_speed: f32,
        pub running_round_duration: f32,
        pub strafschluck_per_game: f32,
        pub strafbeer_per_game: f32,
    }

    // Calculated from the wm2024 without gewertet. The running round duration is simply estimated and used here to check if my method for calculating the running speed is accurate
    pub const JEROME_STATS: PlayerStats = PlayerStats {
        member: JEROME,
        accuracy: 71.43,
        drinking_speed: 2.22,
        running_round_duration: 0.9,
        strafschluck_per_game: 0.12,
        strafbeer_per_game: 0.0,
    };
    pub const BEEF_STATS: PlayerStats = PlayerStats {
        member: BEEF,
        accuracy: 66.67,
        drinking_speed: 2.96,
        running_round_duration: 1.1,
        strafschluck_per_game: 0.12,
        strafbeer_per_game: 0.12,
    };
    pub const SEBI_STATS: PlayerStats = PlayerStats {
        member: SEBI,
        accuracy: 66.67,
        drinking_speed: 3.71,
        running_round_duration: 1.1,
        strafschluck_per_game: 0.0,
        strafbeer_per_game: 0.0,
    };
    pub const FLO_STATS: PlayerStats = PlayerStats {
        member: FLO,
        accuracy: 52.94,
        drinking_speed: 2.5,
        running_round_duration: 0.9,
        strafschluck_per_game: 0.25,
        strafbeer_per_game: 0.0,
    };
    pub const LUISE_STATS: PlayerStats = PlayerStats {
        member: LUISE,
        accuracy: 33.33,
        drinking_speed: 3.72,
        running_round_duration: 1.1,
        strafschluck_per_game: 0.33,
        strafbeer_per_game: 0.0,
    };
    pub const TOBIAS_STATS: PlayerStats = PlayerStats {
        member: TOBIAS,
        accuracy: 30.77,
        drinking_speed: 2.6,
        running_round_duration: 1.0,
        strafschluck_per_game: 0.0,
        strafbeer_per_game: 0.11,
    };
    pub const CHRIS_STATS: PlayerStats = PlayerStats {
        member: CHRIS,
        accuracy: 47.62,
        drinking_speed: 3.27,
        running_round_duration: 1.0,
        strafschluck_per_game: 0.00,
        strafbeer_per_game: 0.00,
    };
    pub const MALTE_STATS: PlayerStats = PlayerStats {
        member: MALTE,
        accuracy: 27.27,
        drinking_speed: 3.25,
        running_round_duration: 1.0,
        strafschluck_per_game: 0.22,
        strafbeer_per_game: 0.00,
    };
    pub const SASCHA_STATS: PlayerStats = PlayerStats {
        member: SASCHA,
        accuracy: 50.0,
        drinking_speed: 2.8,
        running_round_duration: 0.9,
        strafschluck_per_game: 0.12,
        strafbeer_per_game: 0.12,
    };
    pub const JONAS_STATS: PlayerStats = PlayerStats {
        member: JONAS,
        accuracy: 36.84,
        drinking_speed: 3.0,
        running_round_duration: 1.0,
        strafschluck_per_game: 0.38,
        strafbeer_per_game: 0.00,
    };

    pub struct TeamStats {
        pub first: PlayerStats,
        pub second: PlayerStats,
    }

    pub const DA_HAM_SIE_STATS: TeamStats = TeamStats {
        first: JEROME_STATS,
        second: BEEF_STATS,
    };
    pub const DOS_BROS_STATS: TeamStats = TeamStats {
        first: FLO_STATS,
        second: SEBI_STATS,
    };
    pub const STRAMMSEIN_STATS: TeamStats = TeamStats {
        first: SASCHA_STATS,
        second: JONAS_STATS,
    };
    pub const WHITE_CLAW_STATS: TeamStats = TeamStats {
        first: LUISE_STATS,
        second: TOBIAS_STATS,
    };
    pub const WEDELMEDEL_STATS: TeamStats = TeamStats {
        first: CHRIS_STATS,
        second: MALTE_STATS,
    };
}
