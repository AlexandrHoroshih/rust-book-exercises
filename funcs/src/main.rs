fn main() {
    my_custom_func("Hello from custom func!", plus_one(6));
}

fn my_custom_func(somestring: &str, number: i32) {
    println!("My string is \"{}\", my number is {}", somestring, number);
}

fn plus_one(x: i32) -> i32 {
    // last expression in func is returns
    // if we add ; at the end - then it will be statement that does not return value
    x + 1
}
