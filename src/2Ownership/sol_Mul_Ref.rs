fn main() {
    let mut s = String::from("Hello");

    print_string(&mut s); // mutable borrow
    println!("After function call: {}", s);
}

fn print_string(s1: &mut String) {
    s1.push_str(" World");
    println!("Inside function: {}", s1);
}

Ownership main के पास ही रही
Heap data modify हुआ
Same heap, no clone

Stack                    Heap
-----                    -----
s  ────────────────▶    "Hello World"
s1 ──mut ref────────▲
Rule:
✔️ No clone
✔️ Heap reuse
✔️ Modification allowed
⚠️ Rule: उस समय और कोई borrow नहीं होना चाहिए


