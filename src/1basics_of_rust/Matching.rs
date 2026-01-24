fn main() {
    let number = 2;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
    match number {
        1..=5 => println!("Between 1 and 5"),
        _     => println!("Greater than 5"),
    }
    match number {
        1 | 2 => println!("One or Two"),
        3     => println!("Three"),
        _     => println!("Other"),
    }

    let x = 10;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Something else"),
    }

    //9️⃣ Match Guards (Extra Condition)
    let x = 4;

    match x {
        n if n % 2 == 0 => println!("Even"),
        _               => println!("Odd"),
    }

    // 🔟 Returning Values from match
    let x = 3;
    let y = match x {
        1 => 10,
        2 => 20,
        _ => 30,
    };
    println!("{}", y); // 30



}
