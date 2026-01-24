use std::io;
use std::thread;// stopper
use std::time::Duration;//unit of time

fn main(){

    let mut name:String=String::new();
    let mut wallet:u32=0;
    loop{
        println!("Hey, How can i help you -register,withdraw,deposit");
        let mut op:String=String::new();
        io::stdin().read_line(&mut op).expect("Input Error.");
        let op=op.trim();
        match op{
            "register"=>register(&mut name),
            "withdraw"=>withdraw(&name,&mut wallet),
            "deposit"=> deposit(&mut wallet),
            "exit"=>{
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Invalid command, try again.");
                continue;
            }

        }
    }

}



fn register( name:&mut String){
    println!("Welcome Customer");
    let mut _name:String=String::new();
    println!("Tell me your name.");
    io::stdin().read_line(&mut _name).expect("Eror in input name");
    *name=_name;// Auto Dereferencing.
}

fn deposit(wallet: &mut u32){
    println!("How much do you want to deposit?");
    let mut deposit_wallet:String=String::new(); 
    io::stdin().read_line(&mut deposit_wallet).expect("Eror in input deposit");
    let mut deposit_wallet:u32= deposit_wallet.trim().parse().expect("Eror");
    *wallet=deposit_wallet;
}

fn withdraw(name:&String, wallet:&mut u32 ){
    if *wallet == 0 {
        println!("You have no money in your wallet!");
        return;
    }
     println!("How much do you want to withdraw?");
    let mut withdraw_money:String=String::new();

    io::stdin().read_line(&mut withdraw_money).expect("Eror in withdraw money");
    
    let mut withdraw_money=withdraw_money.trim().parse().expect("Error");

    if withdraw_money > *wallet {
        println!("You do not have enough money. Your balance is {}.", *wallet);
        return;
    }
    println!("Processing withdrawal...");
    *wallet-=withdraw_money;
    while withdraw_money>0 {
        thread::sleep(Duration::from_secs(1));
        
        if withdraw_money > 1000{
            withdraw_money-=1000;
            println!("Cashing 1000");
        }else{
            println!("Cashing {}",withdraw_money);
            withdraw_money=0;
        }
    }

    println!("Withdrawal complete. Remaining balance: {}", wallet);
}
