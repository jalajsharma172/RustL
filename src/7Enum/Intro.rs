// Simple Enum
// Optional Enum
// Result Enum




fn main() {
    let c:Color=Color::Yellow;
    match c{
        Color::Red => println!(" Red "),
        Color::Yellow => println!("Yellow "),
        Color::Green => println!("Green ")
    }

    let some_value:OptionalValue<i32> =OptionalValue::Some(42);
    let non_value:OptionalValue<i32> = OptionalValue::None;

    match some_value{
        OptionalValue::Some( x )=> println!("Some Value - {} ",x),
        OptionalValue::None => println!("None Input.")
    }
    match non_value{
        OptionalValue::Some( x )=> println!("{}",x),
        OptionalValue::None => println!(" None Input ")
    }

    match divide(10,2){
        Ok(ans)=> {
            println!("ans {} ",ans);
        }
        Err(error)=>{
            println!("error is {}",error);
        }
    }

}


enum Color{
    Red,
    Yellow,
    Green
}




enum OptionalValue<T> {
    Some(T),
    None,
}

// enum Result<T,E>{
//     OK(T),
//     Err(E)
// }

fn divide(x:i32,y:i32)-> Result<i32,String> {
    if y==0 {
        Err("Nothing greate caught errror y shoudn't be zero".to_string())
    }else{
        Ok(x/y)
    }
       
}