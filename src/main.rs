// use rand::Rng;

use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

use std::env;

use std::process;

use hello_guessing_game::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1);
    });

    println!("{}", config.query);
    println!("{}", config.filename);

    if let Err(err) = hello_guessing_game::run(config) {
        eprintln!("{}", err);
        process::exit(1);
    };
}

fn guessing_game() {
    let random = rand::thread_rng().gen_range(1..101);
    println!("random: {}", random);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faild");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
