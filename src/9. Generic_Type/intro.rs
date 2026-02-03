
fn printval<T: std::fmt::Display>(val:T){
    println!("{}",val)
}



fn main(){
    let num:i32=55;
    printval(num);
    let str:String=String::from("jalaj");
    printval(str);
}