fn main() {
    print_message();
    print_msg_with_parameter(42);
    let result = print_msg_with_parameter_with_return(21);
    println!("Result from function with return: {}", result);
}


fn print_message() {
    println!("Hello, world!");
}
fn print_msg_with_parameter(item:u8) {
    // This function is intentionally left unused to demonstrate dead code.
    println!("Hello, number: {}", item);
}

fn print_msg_with_parameter_with_return(item:u8)-> u8 {
    // This function is intentionally left unused to demonstrate dead code.
    println!("Hello, number: {}", item);
    
    return item*2;
}

