use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Input your guess! ☜(˚▽˚)☞");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read the line!");

    println!("Your guess: {}", guess);

}
