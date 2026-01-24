fn main() {
    // String Literal
    let name="Jalaj";
    println!("Hello, {} .", name);
    // String Type
    let mut my_string : String = String::from("Hello");
    my_string.push_str(", ");
    my_string.push_str("Jalaj .");
    println!("{}", my_string);

}


