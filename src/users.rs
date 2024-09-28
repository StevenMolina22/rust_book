use std::{cmp::Ordering, io};

use rand::Rng;


pub fn play_guessing() {
    println!("--- WELCOME TO THE USERS CRATE ---");
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=10);
    println!("Please input your guess.");

    // run till user inputs correct number
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // re declare (overshadowing);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // comparison
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub fn mut_array(arr: [i32; 5]) {
    println!("--- WELCOME TO THE USERS CRATE ---");
    println!("Enter an array index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number 😡");

    let element = arr[index];

    println!("Element was: {element}");
}