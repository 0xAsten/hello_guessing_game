// use rand::Rng;

use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io;

use std::env;
use std::fs;

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        print!("{}", err);
        process::exit(1);
    });

    println!("{}", config.query);
    println!("{}", config.filename);

    if let Err(err) = run(config) {
        println!("{}", err);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
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
