fn main() {
    println!("Hello, world!");

    let number = 5;

    // if-else
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was what?")
    } else {
        println!("condition was false");
    }

    // if-else is an expression, thus it returns value and can be used like this
    let condition = true;
    let number = if condition { 5 } else { 6 }; // type mismatch here is not allowed

    println!("The value of number is: {}", number);

    // loops
    let mut count = 0;
    let loop_result = loop {
        println!("again!");
        count = count + 1;

        if count == 7 {
            println!("loop is done, count is {}", count);
            break "i am a loop result";
        }
    };

    println!("loop resulted with {}", loop_result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("while condition is done, go LIFTOFF!!!");

    let tupa = [10, 20, 30, 40, 50];

    for item in tupa.iter() {
        println!("the array value is: {}", item);
    }

    // (1..8) is Range, rev() - iterate in reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
