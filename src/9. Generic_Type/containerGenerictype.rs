struct container<T>{
    value:T
}
impl<T> container<T>{
    fn get(&self)->&T{// clone ya fir reference
        &self.value
    }
    fn set(&mut self,new_value:T){
        self.value=new_value;
    }

}
fn main(){
    let mut c:container<String>=container{
        value:String::from("jalaj")
    };
    let a=c.get().clone();
    c.set(String::from("kanak"));
    println!("Value is  {} ",c.get());
    println!("Value is  {} ",c.get());
    
    println!("Value is  {} ",a);
    
}