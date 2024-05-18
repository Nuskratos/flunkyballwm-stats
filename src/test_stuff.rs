use rand::Rng;
use crate::calculation::percentage;

struct TestGameFirstThrow {
    first_hit: bool,
    won: bool,
}

// returns a pair value containing
fn test_generate_game(hits_finished: u32, throw_percentage: Option<u32>) -> TestGameFirstThrow {
    let mut iteration = 1;
    let mut first_hits = 0;
    let mut second_hits = 0;
    let mut first_throw_hits = false;
    let mut first_throw_won = false;
    let mut first_percentage = 0;
    let mut second_percentage = 0;
    if throw_percentage.is_none() {
        first_percentage = rand::thread_rng().gen_range(3..=6) * 10;
        second_percentage = rand::thread_rng().gen_range(3..=6) * 10;
    }
    while iteration < 100 {
        let guess_hit = rand::thread_rng().gen_range(1..=100);
        let mut current_throw_percentage = 0;
        if throw_percentage.is_some() {
            current_throw_percentage = throw_percentage.unwrap();
        } else {
            if (1+iteration) % 2 == 0 {
                current_throw_percentage = first_percentage;
            } else {
                current_throw_percentage = second_percentage;
            }
        }
        if guess_hit <= current_throw_percentage {
            if iteration + 1 % 2 == 0 {
                if iteration == 1 {
                    first_throw_hits = true;
                }
                first_hits += 1;
            } else {
                second_hits += 1;
            }
        }
        if first_hits >= hits_finished {
            first_throw_won = true;
            break;
        }
        if second_hits >= hits_finished {
            break;
        }
        iteration += 1;
    }
    TestGameFirstThrow { first_hit: first_throw_hits, won: first_throw_won }
}

pub fn test_first_throw_value(hits_finished: u32, throw_percentage: Option<u32>) {
    let mut wins = 0;
    let mut first_throw_hit = 0;
    let mut first_hit_win = 0;
    let mut first_miss_win = 0;
    let iterations = 10000;
    for _ in 1..=iterations {
        let game = test_generate_game(hits_finished, throw_percentage);
        if game.won {
            wins += 1;
            if game.first_hit {
                first_hit_win += 1;
            } else {
                first_miss_win += 1;
            }
        }
        if game.first_hit {
            first_throw_hit += 1;
        }
    }
    let throw_str :String = if throw_percentage.is_none() {String::from("variabler")}else { format!("{}", throw_percentage.unwrap()) };
    println!();
    println!("Test daten mit {}% trefferchance & fertig in {} Treffern:", throw_str, hits_finished);
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht {} mal gewonnen. Das sind {:.1}%", iterations, wins, percentage(wins, iterations));
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst getroffen. Dabei {} mal gewonnen. Das sind {:.1}%", first_throw_hit, first_hit_win, percentage(first_hit_win, first_throw_hit));
    println!("In {} Spielen hat das Team mit dem 1. Wurfrecht zuerst verfehlt. Dabei {} mal gewonnen. Das sind {:.1}%", iterations - first_throw_hit, first_miss_win, percentage(first_miss_win, iterations - first_throw_hit));
}
