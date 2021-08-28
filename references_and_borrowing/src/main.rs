fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // woo! this func did not take ownership, &s1 - means "use s1 as a reference, do not take ownership"
                                     // still can use s1 here!
    println!("{}", s1);
    // difference is reference will not call Drop, when out of scope

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    change(&s);

    let mut s = String::from("hello");

    mut_change(&mut s);

    // woo! _ prefix suppresses warnings on vars
    let _r1 = &mut s;
    // wow! cant do that! there is must be only one mutable reference to value on scope!
    // thanks to that we prevent races in compile time!
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    // only one mutable reference at the time!
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 // woo! mutable reference must be **the only** reference in scope!
                 // because users of immutable ref wont expect it to change!

    println!(
        "{} string, has refs: {}, {}, and no mut ref, cause we already have immutable ones",
        s, r1, r2
    );

    // Dangling references
    // let reference_to_nothing = dangle();
    // woo! error!
    let s = no_dangle();
    // all good
    println!("{}", s);

    // so, in the end
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
}
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &String) {
    // woo! does not work. we *borrowed* this value from its owner and we have to return it the way it was
    // some_string.push_str(", world");
    println!("can't do anything with {}", some_string)
}

fn mut_change(some_string: &mut String) {
    // woo! we *can* mutate borrowed value now! because we explictily allowed it with &mut (mutable reference)
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
