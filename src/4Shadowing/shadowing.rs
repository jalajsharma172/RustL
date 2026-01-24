

fn main(){
    let x=5;// U8
    println!("Original Value {}",x);

    let x="Hello"; // STRING
    println!("New Value {}",x);

    let x=x.len(); // USIZE
    println!("Length of x {}",x)

}

Shadowing क्या होता है?
Same name का नया variable बनाना,जो पुराने variable को temporarily hide (shadow) कर देता है।

fn main(){
   let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);
}
✔️ Same name
✔️ Different type
❌ mut से impossible



5️⃣ Shadowing in Scope (Very Important)
fn main() {
    let x = 5;

    {
        let x = x + 10; // inner scope shadow
        println!("{}", x); // 15
    }

    println!("{}", x); // 5
}


