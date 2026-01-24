 mod math {//priavte liib hai
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

use math::add;
// use math::*;
// mod FILENAME
// use FILENAME::main::add;
 fn main() {
    // let sum = math::add(2, 3);
    let sum = add(2, 3);
    // let subs=math::sub(5,1);// Sub is PRIVATE
    println!("{}", sum);
    // println!("{}", subs);
}

