use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::time::Duration;
use std::collections::HashSet;

enum Showroom {
    AVAILABLE,
    BUSY
}

struct Guest {
    id: usize,
    showroom: Arc<Mutex<Showroom>>,
    visited: bool
}

impl Guest {
    fn enter_showroom(&self) -> bool {
        let mut status = self.showroom.lock().unwrap();
        let res = match *status {
            Showroom::AVAILABLE => {
                *status = Showroom::BUSY;
                true
            },
            Showroom::BUSY => false
        };
        res
    }

    fn leave_showroom(&mut self) {
        let mut status = self.showroom.lock().unwrap();
        *status = Showroom::AVAILABLE;
        self.visited = true;
    }
}

pub fn execute_problem_2(num_threads: usize) {

    println!("Problem 2: Minotaur's Crystal Vase\n");

    let mut guests = vec![];

    let showroom = Arc::new(Mutex::new(Showroom::AVAILABLE));
    let visitor_log = Arc::new(Mutex::new(HashSet::new()));

    let start = Instant::now();

    for i in 0..num_threads {
        let mut guest = Guest {id: i, showroom: Arc::clone(&showroom), visited: false};
        let visitor_log = Arc::clone(&visitor_log);
        guests.push(thread::spawn(move || {
           loop {
               if guest.enter_showroom() {
                   let mut log = visitor_log.lock().unwrap();

                   println!("Guest # {} has entered the showroom", guest.id);

                   thread::sleep(Duration::from_millis(50));

                   guest.leave_showroom();

                   log.insert(guest.id);
                   println!("Guest # {} has left the showroom", guest.id);

                   if log.len() == num_threads {
                       break;
                   }
               }
           }
        }));
    }

    for guest in guests {
        guest.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time to execute protocol: {:?}", duration);
}