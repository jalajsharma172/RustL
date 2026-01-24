use std::io;
use std::time::Duration;// Duration → to represent time Unit
use std::thread;// thread::sleep() → to wait 1 second
// loop → to keep counting down
fn main() {
    println!("Enter number of seconds:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let mut second: u64 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    while second>0 {
        println!("⏳ Time left: {} seconds", second);
        thread::sleep(Duration::from_secs(1));
        second=second-1;
    }

}
