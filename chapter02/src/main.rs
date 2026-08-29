// Chapter02: Guessing Game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main1() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}");

    let x = 5;
    let y = 6;
    println!("x: {x} y: {y}");
}

fn main3() {
    println!("Guess the number!");

    let securt_number = rand::thread_rng().gen_range(1..=100);
    println!("securt_number is {securt_number}");
    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("guess {guess}");
}

fn main() {
    println!("Guess the number!");

    let securt_number = rand::thread_rng().gen_range(1..=100);
    println!("securt_number is {securt_number}");
    println!("Please input your guess:");

    let mut guess_str = String::new();
    io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line.");

    let guess: u32 = guess_str.trim().parse().expect("Please type a number!");

    println!("you guessed : {guess}");

    match guess.cmp(&securt_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("Yow Win"),
    }
}
