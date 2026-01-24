// #[derive(Debug)]
struct Student{
    name:String,
    age:u8
}
fn main(){
    let std:Student=Student{
        name:String::from("Jalaj"),
        age:20
    };
    // println!("{:#?}",std);
    // println!("Name : {:#?}",std.name);
    // println!("Age  : {:#?}",std.age);
    println!("Hey man");
}