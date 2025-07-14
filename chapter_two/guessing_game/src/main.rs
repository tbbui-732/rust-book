use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let min_range: u32 = 1;
    let max_range: u32 = 100;
    let secret_number: u32 = rand::rng()
                            .random_range(min_range..=max_range);

    println!("guess a number between {min_range} to {max_range}");
    println!("input: ");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please only enter integer values!");
                continue;
            },
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("that's the right answer!");
                break;
            },
        }
    }
}
