use std::io;
fn add (a:f64,b:f64)->f64{
    a+b
}
fn sub (a:f64,b:f64)->f64{
    a-b
}
fn mul (a:f64,b:f64)->f64{
    a*b
}
fn div (a:f64,b:f64)->f64{
    if b==0.0 {return 0.0;};
    a/b
}
fn sqrt(a: f64) -> f64 {
    a.sqrt()
}

fn main (){
    loop{
            println!("Enter opertion name -add,sub,mul,div,exit . ");
    let mut op=String::new();
    io::stdin().read_line(&mut op).expect("Invalid Operation");
    let op=op.trim();

    if op =="exit" {return;}

    let mut a=String::new();
    let mut b=String::new(); 
    println!("Enter First Number: ");
    io::stdin().read_line(&mut a).expect("Invalid Number");
    println!("Enter Second Number: ");
    io::stdin().read_line(&mut b).expect("Invalid Number");
    let a:f64=a.trim().parse().expect("Invalid");
    let b:f64=b.trim().parse().expect("Invalid");
    
    let result = match op {
        "add" => add(a,b),
        "sub"=> sub(a,b),
        "mul"=> mul(a,b),
        "div"=> div(a,b),
        "sqrt"=>sqrt(a);
        _ => {
            println!("Invalid operation");
            return;
        }
    };
    println!("Result is {} ",result)
    }

}