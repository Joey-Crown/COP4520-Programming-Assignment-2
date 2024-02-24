use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use rand::Rng;

enum Cupcake {
    Present,
    Absent
}

struct Labyrinth {
    cupake: Cupcake,
    count: usize,
    current_visitor: usize,
}

impl Labyrinth {
    fn next_guest(&mut self, val: usize) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..val);
        self.current_visitor = num;
    }
}

trait Guest {
    fn enter_labyrinth(&mut self, num_guests: usize);
}

struct Leader {
    id: usize,
    labyrinth: Arc<Mutex<Labyrinth>>
}

impl Guest for Leader {
    fn enter_labyrinth(&mut self, num_guests: usize) {
        loop {
            let mut _labyrinth = self.labyrinth.lock().unwrap();

            if _labyrinth.current_visitor == self.id {
                println!("Guest # {} has entered the Labyrinth.", self.id);
                match _labyrinth.cupake {
                    Cupcake::Absent => {
                        println!("Guest # {} (Leader) has Replaced the Cupcake.", self.id);
                        _labyrinth.cupake = Cupcake::Present;
                        _labyrinth.count += 1;
                    },
                    _ => ()
                }
                _labyrinth.next_guest(num_guests);
            }

            match _labyrinth.count < num_guests - 1 {
                true => continue,
                _ => break
            }

        }
    }
}

pub struct Follower {
    id: usize,
    labyrinth: Arc<Mutex<Labyrinth>>,
    eaten: bool
}

impl Guest for Follower {
    fn enter_labyrinth(&mut self, num_guests: usize) {
        loop {
            let mut _labyrinth = self.labyrinth.lock().unwrap();

            if _labyrinth.current_visitor == self.id {
                println!("Guest # {} has entered the Labyrinth", self.id);
                match _labyrinth.cupake {
                    Cupcake::Present if !self.eaten => {
                        println!("Guest # {} (Follower) has Eaten the Cupcake.", self.id);
                        _labyrinth.cupake = Cupcake::Absent;
                        self.eaten = true;
                    },
                    _ => ()
                }
                _labyrinth.next_guest(num_guests);
            }

            match _labyrinth.count < num_guests - 1 {
                true => continue,
                _ => break
            }
        }
    }
}

pub fn execute_problem_1(num_threads: usize) {
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

    println!("Problem 1: Minotaur's Crystal Vase\n");

    let mut threads = vec![];
    let labyrinth = Arc::new(
        Mutex::new(Labyrinth { cupake: Cupcake::Present, count: 0, current_visitor: 0}));

    let start = Instant::now();

    let mut leader = Leader {id: 0, labyrinth: Arc::clone(&labyrinth)};
    threads.push( thread::spawn(move || {
        leader.enter_labyrinth(num_threads);
    }));

    for i in 1..num_threads {
        let mut follower = Follower {id: i, labyrinth: Arc::clone(&labyrinth), eaten: false};
        threads.push(thread::spawn(move || {
            follower.enter_labyrinth(num_threads);
        }));
    }

    for child in threads {
        child.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time to execute protocol: {:?}", duration);
}