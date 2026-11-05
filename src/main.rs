/*
By: Hassan Rana
Date: 2026-10-02
Program Details: A program that is a fun text based magic 8 ball that gives you a random answer to your question.
*/
use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Magic 8 Ball! Enter in a question to get a random answer. Type 'exit' to quit the program.");
    loop {
        let mut input = String::new();
        println!("Enter your question:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "exit" {
            break;
        }
        let mut rng = rand::rng();
        let dice: i32 = rng.random_range(1..=8);
        match dice {
            1 => println!("Yes, definitely!"),
            2 => println!("No, definitely not!"),
            3 => println!("Maybe, who knows?"),
            4 => println!("It is certain."),
            5 => println!("Roads lead to no."),
            6 => println!("Don't count on it."),
            7 => println!("Outlook not so good."),
            8 => println!("Signs point to yes."),
            _ => println!("Invalid dice roll."),
        }
    }
}
