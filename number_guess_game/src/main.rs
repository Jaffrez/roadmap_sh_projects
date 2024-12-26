use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn main() {
    // * Show some introduction to the game
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have some chances to guess the correct number.\n\n");

    println!("Please select the difficulty level:");
    println!("1. Easy(10 chances)");
    println!("2. Medium (5 chances)");
    println!("3. Hard (3 chances)");

    print!("Please enter your choice:");
    io::stdout().flush().unwrap();
    let choice = {
        let mut buf = String::new();
        loop {
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line");
            match buf.trim().parse::<u32>() {
                Ok(num) => {
                    if num > 0 && num < 4 {
                        break num;
                    } else {
                        println!("Please enter a valid choice:");
                        buf.clear();
                    }
                }
                Err(_) => {
                    println!("Please enter a valid choice:");
                    buf.clear();
                }
            }
        }
    };
    println!("Let's start the game!");

    // * Generate a number
    let number: u32 = thread_rng().gen_range(1..=100);
    let mut i = 0;

    loop {
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Ok(_) => {}
                Err(_) => {
                    println!("Please try again!");
                    continue;
                }
            }
            match buf.trim().parse::<u32>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Please try again!");
                    continue;
                }
            }
        };

        if input == number {
            match choice {
                1 => {
                    if i > 10 {
                        println!("You have used all your chances.");
                        break;
                    }
                }
                2 => {
                    if i > 5 {
                        println!("You have used all your chances.");
                        break;
                    }
                }
                3 => {
                    if i > 3 {
                        println!("You have used all your chances.");
                        break;
                    }
                }
                _ => unreachable!(),
            }
            println!(
                "Congratulations! You guessed the correct number in {} attempts.",
                i
            );
            break;
        } else {
            if input > number {
                println!("Incorrect! The number is less than {}", input);
            } else {
                println!("Incorrect! The number is greater than {}", input);
            }
            i += 1;
        }
    }
}
