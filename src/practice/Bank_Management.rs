struct Account {
    account_number: u32,
    holder_name: String,
    balance: i32,
}
struct Bank {
    name: String,
    account: Option<Account>, // bank can manage only one account
}
impl Account{
        // fn add_account(&self,bank:Bank){
            
        // }
        fn deposit(&mut self,money:i32){
            self.balance+=money;
        }
        fn withdraw(&mut self,money:i32){
            self.balance-=money;
        }
        fn show_balance(&self)->i32{
            self.balance
        }
}
impl Bank{
    fn add_account(&mut self,account:Account){
        self.account=Some(account);        
    }
    fn deposit_to_account(&mut self,money:i32){
            //check
            match &mut self.account{
                Some(account)=>{
                    if money>0{
                         account.deposit(money);
                    }else{
                        println!("Erorr");
                    }
                   
                }
                None=>{
                    println!("Error ");
                }
            }
    }
    fn withdraw_from_account(&mut self,money:i32){
        match &mut self.account{
            Some(account)=>{
                if account.balance>=money{
                    account.withdraw(money);
                }else{
                    println!("Insufficient Balance");
                }
            }
            None=>{
                println!("Error while withdraw");
            }
        }
    }
    fn show_balance(&mut self){
        match &mut self.account{
            Some(account)=>{
                let balance=account.show_balance();
                println!("Balance : {}",balance);
            }
            None=>{
                println!("Error");
            }
        }
    }
}
fn main(){
    let account:Account=Account{
        account_number:10,
        holder_name:"Jalaj".to_string(),
        balance:0
    };
    let mut bank:Bank=Bank{
        name:"JJ".to_string(),
        account:None
    };
    bank.add_account(account);
    bank.deposit_to_account(1000);
    bank.deposit_to_account(1000);
    bank.deposit_to_account(1000);
    bank.withdraw_from_account(100);
    bank.show_balance();
}