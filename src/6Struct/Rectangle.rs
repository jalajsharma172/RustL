



struct Rectangle{
    length:u16,
    width:u16
}

fn main(){
    let mut rec:Rectangle=Rectangle{
        length:26,
        width:33
    };
    let ans = area(&rec);
    println!("{}",ans);
    double(&mut rec);
    println!("{}",rec.length);
    println!("{}",rec.width); 
}

fn area(rec:&Rectangle)->u16{
    rec.length*rec.width
}
fn double(rec: &mut Rectangle ){
    rec.length=rec.length*2;
    rec.width=rec.width*2;
}










// #[derive(Debug)]
// struct Student{
//     name:String,
//     age:u8
// }
// fn main(){
//     let std:Student=Student{
//         name:String::from("Jalaj"),
//         age:20
//     };
//     println!("{:#?}",std);
//     println!("Name : {:#?}",std.name);
//     println!("Age  : {:#?}",std.age);
//     println!("Hey man");
// }