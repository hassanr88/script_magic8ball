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
        let ans1 = "Yes, definitely!";
        let ans2 = "No, definitely not!";
        let ans3 = "Maybe, who knows?";
        let ans5 = "It is certain.";
        let ans4 = "Roads lead to no.";
        let ans6 = "Don't count on it.";
        let ans7 = "Outlook not so good.";
        let ans8 = "Signs point to yes.";
        if input.trim().to_lowercase() == "exit" {
            break;
        }
        let mut rng = rand::rng();
        let dice: i32 = rng.random_range(1..=8);
        match dice {
            1 => println!("{}", ans1),
            2 => println!("{}", ans2),
            3 => println!("{}", ans3),
            4 => println!("{}", ans4),
            5 => println!("{}", ans5),
            6 => println!("{}", ans6),
            7 => println!("{}", ans7),
            8 => println!("{}", ans8),
            _ => println!("Invalid dice roll."),
        }
    }
}
