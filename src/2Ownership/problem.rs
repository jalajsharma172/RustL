fn main(){
    let str:String =String::from("Hello");
    print_string(str.clone());//str->str_one[STACK]=Hello[HEAP]
    println!("{} is this string",str);//str[stack].pointer=null;
}
fn print_string(str_one:String) {
    println!("{} ->String_one in print_string.",str_one);
}