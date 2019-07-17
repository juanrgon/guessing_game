use rand::Rng;
use std::cmp::Ordering;
use ansi_term::Color::Red;
use ansi_term::Color::Yellow;
use ansi_term::Color::Green;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    print!("Guess the number!");

    loop {
        println!("\nPlease input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", Red.paint("Please input a number!\n"));
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", Yellow.paint("Too small!")),
            Ordering::Greater => println!("{}", Yellow.paint("Too big!")),
            Ordering::Equal => {
                println!("{}", Green.paint("You win!"));
                break;
            },
        }
    }
}
