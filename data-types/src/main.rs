fn main() {
    println!("SCALARS");
    println!("Ints");
    const MAX_I8: i8 = 127;
    const MAX_U8: u8 = 255;
    print!(
        "Max i8 value is {}, while max u8 value is {}",
        MAX_I8, MAX_U8
    );
    println!("If value is out of bounds, panic overflow happens in dev mode, and overflow wrapping (for e.g. 256 as u8 will turn -> 0 and so on) in release mode");
    println!("Floats");
    println!("float types are f32 (single precision) and f64 (default one, double presicion)");
    let x = 2.3; // f64

    let y: f32 = 3.1; // f32
    println!("f64: {}, f32: {}", x, y);

    println!("bools");
    let f: bool = true;
    println!("nothing special with bools, {}", f);
    println!("chars");
    let c = 'z';
    let z = 'ℤ';
    const HEART_EYED_CAT: char = '😻';
    println!("{}, {}, {}", c, z, HEART_EYED_CAT);
    println!("COMPOUNDS!");
    println!("Tuple");
    let tup: (f64, i16, bool) = (5.4, 34, false);
    let (x, y, z) = tup;
    println!(
        "tuple (f64, i16, bool) ({}, {}, {}), last item is tuple.2 = {}",
        x, y, z, tup.2
    );
    println!("Arrays");
    // arrays have fixed length and same type for all items
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Nine month is {}", months[8]);
}
