fn main() {
    // Slice type - special type with no ownership
    // way to reference slice of collection instead of the whole thing
    let mut s = String::from("hello world");

    let word = first_word_naive(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to "", no warning from Rust

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // slices!
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; // slice reference to part of the string!

    let word = first_word(&s);

    // s.clear(); // error! we cannot mutate s, because word keeps immutable ref to it

    println!("the first word is: {}", word);

    // not just for strings
    let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3]; // valid slice ref
}

fn first_word_naive(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
