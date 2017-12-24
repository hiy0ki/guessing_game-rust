extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    println!("input your guess!");

    let mut guess = String::new();

    // &mut はミュータブルな参照
    io::stdin().read_line(&mut guess)
        .expect("Faild to read line");

    println!("You guessed: {}", guess);

    // cmp()は比較したいものはなんでも呼べて、引数に比較対象の参照を取る
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
