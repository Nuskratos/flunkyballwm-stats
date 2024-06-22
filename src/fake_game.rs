#[cfg(test)]
pub mod test {
    use rand::Rng;
    use rand::rngs::ThreadRng;
    use crate::team_player_data::test::{PlayerStats, TeamStats};

    fn mock_game(left: &TeamStats, right: &TeamStats) {
        let mut rand = rand::thread_rng();
        let left_first = rand.gen_range(0..=1) == 1;
        let mut throwers: Vec<&PlayerStats> = Vec::new();
        let mut runners: Vec<&PlayerStats> = Vec::new();
        throwers.push(&left.first);
        throwers.push(&right.first);
        throwers.push(&left.second);
        throwers.push(&right.second);
        runners.push(&right.first);
        runners.push(&left.first);
        runners.push(&right.second);
        runners.push(&left.second);
        let mut iteration = 0;
        while iteration < 1000 {
            let guess_hit = rand.gen_range(0.0..=100.0) <= throwers.first().unwrap().accuracy;


            // reorder the vectors
        }
    }

}