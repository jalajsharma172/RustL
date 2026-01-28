
struct Book {
    title: String,
    author: String,
    is_available: bool
}
struct Library{
    name: String,
    address: String,
    book:Option<Book>// 0 or 1 book 
}
impl Book{
    fn return_book(&mut self){
        self.is_available=false;
    }
    fn borrow_book(&mut self){
        self.is_available=true;
    }
}
impl Library{
    fn add_book(&mut self,book :Book){
        println!("{} Book Addded ",book.title);
        self.book=Some(book);
    }
    fn borrow_book(&mut self){
        // Lib->Book->!bool
        match &mut self.book{// Mutable passing
            Some(book)=>{
                if book.is_available {
                    book.return_book();
                }else{
                    println!("Book is not avaible.");
                }
            }
            None=>{println!("Book not avaiable");}
        }
    }
    fn return_book(&mut self){
        match &mut self.book{
            Some(book)=>{
                if book.is_available{
                    println!("Book Information miss match");
                }else{
                    book.borrow_book();
                }
            }
            None=>{println!("Book not found");}
        }
    }
}

fn main(){
    let mut b:Book=Book{
        title:"Rust book".to_string(),
        author:"MM".to_string(),
        is_available:true,
    };
    let mut l:Library=Library{
        name:"GG".to_string(),
        address :"Greater noida".to_string(),
        book:None
    };
    l.add_book(b);
    l.borrow_book();
    l.borrow_book();
    l.return_book();
    l.return_book();

}
