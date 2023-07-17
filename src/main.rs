// use rand::Rng;

use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
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
