enum Direction{
    North,
    South,
    West,
    East
}
enum Message{
    Quit,
    Write (String),
    Move {x:i32,y:i32}
}
fn main (){
    let d=Direction::East;
    let d2=Direction::South;
    let d3=Direction::West;
    handle_Direction(&d);
    handle_Direction(&d2);
    handle_Direction(&d3);
    let m=Message::Quit;
    let m2=Message::Write(String::from("jjalaj"));
    let m3=Message::Move {x:10,y:20};
    handle_msg(&m);
    handle_msg(&m2);
    handle_msg(&m3);






















}
    fn handle_msg(m:&Message){
        match m{
            Message::Quit=> println!("Quite msg"),
            Message::Write(text)=>println!("msg = {}",text),
            Message::Move{x,y}=>println!(" Move to ({} ,{}) ",x,y),
        }  
    }
    fn handle_Direction(d:&Direction){
        match d{
            Direction::North=>println!("North chalo"),
            Direction::South=>println!("South chalo"),
            Direction::West=>println!("West chalo"),
            Direction::East=>println!("East chalo")
        }
    }

