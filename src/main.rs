extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input your guess!");
        
        let mut guess = String::new();

        // &mut はミュータブルな参照
        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");

        // このままではsecret_numberとguessの型が一致しないのでエラーになるためキャストする。
        // 以前のguessを新しいguessで定義（隠す）することができる。これをシャドーイングという。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp()は比較したいものはなんでも呼べて、引数に比較対象の参照を取る
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
