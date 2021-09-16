use rand::Rng;
use std::{
    io::{self, Write},
    vec,
};

fn main() {
    println!("Rust Dice-Roller!");
    println!("by Lcravo_g");

    //Get user input.
    loop {
        let wtp: bool = true;
        println!("Enter The dices you want to roll.");
        let mut u_input = String::new();
        io::stdin()
            .read_line(&mut u_input)
            .expect("Failed to read the line.");

        println!("===================================");
        roll(&u_input);
        println!("===================================");
    }
}

fn roll(input: &str) {
    let split_inp = input
        .split_whitespace()
        .filter_map(|dice| {
            match dice
                .split('d')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(values) => Some(values),
                Err(e) => {
                    eprintln!("Sorry, invalid input: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<Vec<usize>>>();

    for dice in split_inp {
        print!("{}d{}: ", dice[0], dice[1]);
        for _i in 0..dice[0] {
            print!("{}, ", rand::thread_rng().gen_range(1..dice[1] + 1));
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
    //TODO: Print the results -> 2d10: 10, 5
    //TODO: Ask the user if they like too keep playing.
}
