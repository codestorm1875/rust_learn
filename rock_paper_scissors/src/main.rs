use std::io;
use rand::Rng;

fn main() {
    loop {
        let computer_choice: usize = rand::rng().random_range(1..=3);
        let mut user_choice: String = String::new();
        
        // Take user input: 1 for rock, 2 for paper, 3 for scissors
        println!("1 for rock, 2 for paper, 3 for scissors");
        println!("Input your choice:");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line.");

        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => continue
        };

        match (user_choice, computer_choice as u32) {
            (1, 2) => println!("You lose! paper beats rock."),
            (1, 3) => println!("You win! rock beats scissors."),
            (2, 1) => println!("You win! paper beats rock."),
            (2, 3) => println!("You lose! scissors beats paper."),
            (3, 1) => println!("You lose! rock beats scissors."),
            (3, 2) => println!("You win! scissors beats paper."),
            (a, b) if a == b => println!("Draw"),
            _ => { 
                println!("Invalid choice");
                continue
            }
        }
    }

}
