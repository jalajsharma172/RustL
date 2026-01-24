fn main() {

    let employees : (&str, i32) = ("Alice", 30);    
    println!("Employee Name: {}, Age: {}", employees.0, employees.1);

    // destruction
    let (name, age) = employees;
    println!("Destructured - Name: {}, Age: {}", name, age);

}


