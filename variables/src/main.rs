fn main() {
    // shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    // mutable
    let mut y = 7;
    println!("The value of y is: {}", y);
    y = 8;
    println!("The value of y is: {}", y);
}
