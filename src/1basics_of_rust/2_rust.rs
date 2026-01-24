fn main() {
    let mut num:u8=10; // You can update Value
    let  num2:u8=11;   // You cannot update Value
    println!("1. The value of Mutable num is: {}", num);
    num=25;
    println!("2. The value of Mutable num is: {}", num);
    
    println!("The value of Immutable num is: {}", num2);
    println!("Hello, world!");
}


