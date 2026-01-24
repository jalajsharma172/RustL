use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let mut rang=rand::thread_rng();

    let ran_target:u32=rang.gen_range(1..=100);
    println!("{}",ran_target);

    loop {
        let val=rang.gen_range(1..100);
        match val.cmp(&ran_target){ 
            Ordering::Less=>{println!("{} is Less than {}",val,ran_target);}
            Ordering::Greater=>{println!("{} is Greater than {}",val,ran_target)}
            _=>{ println!("Equals 🎉");   break;      }
        }
    }
}

