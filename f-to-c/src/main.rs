use std::io;

fn main() {
    println!("Farenheit to Celcius converter!");

    loop {
        println!("Choose mode:");
        println!("1 is F to C, 2 is C to F");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Could not read the line! (°ロ°)☝");

        let current_mode: i8 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a correct mode! (°ロ°)☝");
                continue;
            }
        };

        if current_mode == 1 {
            println!("Your mode is F to C");
        } else if current_mode == 2 {
            println!("Your mode is C to F");
        } else {
            println!("Please type a correct mode! (°ロ°)☝");
            continue;
        }
        println!("Enter value to convert");

        let mut val = String::new();

        io::stdin()
            .read_line(&mut val)
            .expect("Could not read the line! (°ロ°)☝");

        let val: f32 = match val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a correct value! (°ロ°)☝");
                continue;
            }
        };

        if current_mode == 1 {
            println!("{} F is {} in C", val, f_to_c(val))
        } else {
            println!("{} C is {} in F", val, c_to_f(val))
        }
    }
}

fn f_to_c(num: f32) -> f32 {
    (num - 32.0) * 5.0 / 9.0
}

fn c_to_f(num: f32) -> f32 {
    (num * 9.0 / 5.0) + 32.0
}
