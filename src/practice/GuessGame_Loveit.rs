use std::io;
use std::cmp::Ordering;//Compare + Ordering

//enum Ordering {
//     Less,
//     Equal,
//     Greater,
// }

fn main(){
      println!("Enter target number:");
    let mut target:String=String::new();
    io::stdin().read_line(&mut target).expect("Invalid Input");

    let target:i32=target.trim().parse().expect("Invalid");
    loop {
        println!("Enter your guess:");

        let mut num=String::new();
            io::stdin().read_line(&mut num).expect("Invalide Guess");//Input
            let num:i32=num.trim().parse().expect("Invalid");        //Parse   

        match num.cmp(&target){
            Ordering::Less => println!("Lesser"),
            Ordering::Equal => { println!("Equals 🎉");   break;            },
            Ordering::Greater => println!("Greater")
        }
    }
}







// cmp - > cmp एक method है जो दो values को compare करता है
//और result में देता है एक enum: Ordering



// | Result    | Meaning |
// | --------- | ------- |
// | `Less`    | a < b   |
// | `Equal`   | a == b  |
// | `Greater` | a > b   |



