
struct Rectangle{
    length:u32,
    width:u32
}

fn main (){
    let mut rec=Rectangle{
        length:10,
        width:30
    };
    let ans=area(&rec);// memory sharing kr rahe hoge
    println!("{}",ans);
    double_parameters(&mut rec);
    let ans =area(&rec);
    println!("{}",ans);

}

fn area(rec:&Rectangle)->u32{
    rec.length*rec.width
}
fn double_parameters(rec:&mut Rectangle){
    rec.width=rec.width*2;
    rec.length=rec.length*2
}
