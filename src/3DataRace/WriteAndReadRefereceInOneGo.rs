Multiple Reference

fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2= &s; 
    
    println!("{}  {}", r1,r2);
    
    let w = &mut s; // Writing baad me
    w.push_str("World.");
    println!("{}",w)
}


fn main() {
    let mut s = String::from("Hello");

    let w = &mut s;// Writing padle
    w.push_str("World.");
    println!("{}",w);
    
    let r1 = &s;
    let r2= &s; 
    
    println!("{}  {}", r1,r2);
    
}



fn main() {
    let mut s = String::from("Hello");
            {
                let w = &mut s;
                w.push_str("World.");
                println!("{}",w)

            }
    let r1 = &s;
    let r2= &s; 
    
    println!("{}  {}", r1,r2);
    
}
HelloWorld.
HelloWorld.  HelloWorld.



