
mod minotaurs_birthday_party;
mod minotaurs_crystal_vase;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rand::Rng;
use minotaurs_birthday_party::execute_problem_1;
use minotaurs_crystal_vase::execute_problem_2;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let arg = match args.get(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments provided!");
            return;
        }
    };

    let num_threads = match arg.parse::<usize>() {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to parse number from argument: {}", e);
            return;
        }
    };

    println!("Executing with {}\n threads", num_threads);

    execute_problem_1(num_threads);
    execute_problem_2(num_threads);

}