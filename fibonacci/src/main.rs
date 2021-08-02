use std::io;

fn main() {
    println!("Calc nth Fibonacci number");
    loop {
        println!("Enter an N!");
        println!("Which fibonacci number you would like to know");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the line! (°ロ°)☝");

        let nth: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("{}", error);
                println!("Please type a correct number! (°ロ°)☝");
                continue;
            }
        };

        if nth > 92 {
            println!("Sorry! This is too much for this program");
            println!("It will cause integer overflow");
            println!("Please enter smaller value");
            continue;
        }

        let result = fib(nth);

        println!("Your {}-th fibonacci number is {}", nth, result);
        println!("\n");
    }
}

fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return n;
    } else if n == 2 {
        return 1;
    }

    let result = fib_fast_double(n);

    return result.0;
}

fn fib_fast_double(n: usize) -> (usize, usize) {
    if n == 0 {
        return (0, 1);
    }

    let (a, b) = fib_fast_double(n / 2);
    let c = a * (b * 2 - a);
    let d = a * a + b * b;

    if n % 2 == 0 {
        return (c, d);
    }

    return (d, c + d);
}
