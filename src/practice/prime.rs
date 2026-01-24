fn main(){
     let  num:u32=432;
     let _status:bool=is_prime(num);
     if _status {
          print!("{} is a prime number",num);
     }else{
          print!("{} is not a prime number",num)
     }
}

fn is_prime(_num:u32 )->bool{
     if _num<=1 {
        return false;
    }
     if _num<=3 {
        return true;
    }
    if _num%2==0 || _num%3==0 {
        return false;
    }
     let mut i:u32=5;
     while i<_num {
          if _num%i ==0 {return true}
          i+=1;
     }
     return false;
     }
     