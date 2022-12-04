use std::io::Write;

use chrono::{Timelike, Utc};
use tdd_katas::ohce_kata::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("ohce take only one argument");
    }

    let name = args.get(1).unwrap();
    let ohce = Ohce::start(name, ChronoTimeProvider::default());

    println!("$ ohce {name}");
    println!("> {}", ohce.greetings);

    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        ohce.echo(&input)
            .iter()
            .for_each(|response| println!("> {response}"));

        if input == "Stop!" {
            break;
        }
    }
}

struct ChronoTimeProvider(u8);

impl Default for ChronoTimeProvider {
    fn default() -> Self {
        Self(Utc::now().time().hour().try_into().unwrap())
    }
}

impl TimeProvider for ChronoTimeProvider {
    fn now(&self) -> u8 {
        self.0
    }
}
