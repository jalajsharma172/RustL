
fn main() {
    let  s = String::from("Hello");//x → actual value
    let x=&s;       //r → reference (address)
    // let add=*s;     //*r → value at that address
    println!("{}", x);//Auto Dereferecing
    println!("{}",*x)//Manually fetching value from reference
    
}


fn main(){
    let mut x=5;
    let r= &mut x;
    *r=20;
    println!("   {}  ",r );
    println!("   {}  ",x );
    
}



fn main(){
    println!(" Multiple Pointer * at once");
    let x=10;
    let r1=&x;
    let r2=&r1;
    let r3=&r2;
    println!(" Read1={} Read2={} Read3={}  ",*r1,**r2,**r3);
}


What is Auto Dereferecing
fn main(){
    let str:String=String::from("hello");
    let len:usize=calculate_length(&str);
    println!("{}",len);
}
fn calculate_length(s2:String)->usize{//Auto Dereferencing
    s2.len()
}
// fn calculate_length2(s2:String)->usize{//Manual Dereferencing
//     (*s2).len()
// }