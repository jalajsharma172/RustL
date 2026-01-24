fn main() {
    let s = String::from("Hello");

    print_string(&s); // borrow
    println!("After function call: {}", s);
}

fn print_string(s1: &String) {
    println!("Inside function: {}", s1);
}

Ownership transfer नहीं हुई

Heap data same है

Function सिर्फ read कर सकता है

Stack                    Heap
-----                    -----
s  ────────────────▶    "Hello"
s1 ──ref────────────▲


