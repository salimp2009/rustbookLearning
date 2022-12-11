use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
    println!("secret number : {secret_num}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_bytes_read) => {
                //println!("bytes read : {bytes_read}")
            }
            Err(_error) => {
                println!("Failed to read line");
            }
        }

        println!("you guessed: {guess}");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => {
                println!("Too small !!");
            }
            Ordering::Equal => {
                println!("Winner  ");
                break;
            }
            Ordering::Greater => {
                println!("Too big !!");
            }
        }
    }
}
