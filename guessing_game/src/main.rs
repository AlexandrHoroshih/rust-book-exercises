use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);


    loop {
        println!("Input your guess! ☜(˚▽˚)☞");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line! (°ロ°)☝");
    
        println!("Your guess ( ͡° ͜ʖ ͡°) {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number! (°ロ°)☝");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small ¯\\_(ツ)_/¯"),
            Ordering::Greater => println!("Too big ( ͡° ͜ʖ ͡°)"),
            Ordering::Equal => {
                println!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ You win! ✧ﾟ･: *ヽ(◕ヮ◕ヽ)");
                break;
            }
        }
    }
}
