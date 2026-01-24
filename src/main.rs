use std::io;
struct Book{//Struct = data
    title: String,
    author:String,
    is_available:bool
}
struct Library {
    name:String,
    address:String,
    book:Option<Book>// ye bolta hai ki Some(Book namak property dal sakte ho) or None
}
enum LibraryError{
    BookNotAvailable,
    BookNotFound,
    AlreadyBorrowed
}

impl Book{//impl = behavior
    fn borrow(&mut Book)->Result<(),String>{
        if Book.is_available==true {
            Book.is_available=false;
            Ok(())
        }else{
            Err(LibraryError::AlreadyBorrowed)
        }
    }
    fn return_book(&mut self)->Result<(),String>{
        if self.is_available==false {
            self.is_available=true;
            Ok(())
        }else{
            Err(LibraryError::BookNotFound)
        }
    }
}
impl Library{
    fn add_book(&mut self,book:Book){
        self.book=Some(book);// Library.book=Some(Book)

    }
    fn borrow_book(&mut self)->Result<(),LibraryError>{
        
    }
    fn return_book(){}  
}


fn main(){
    let book:Book=Book{
        title:"Rust",
        author:"None",
        is_available:true
    }   
    let library:Library=Library{
        name:"crytal",
        address:"XYZ",
        book:None   
    }
}

