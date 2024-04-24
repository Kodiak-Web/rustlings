// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    Move {x:i16,y:i16},
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move{x,y} => println!("Moved +{}, +{}",x,y),
            Message::Echo(mystr) => println!("{}",mystr),
            Message::ChangeColor(r,g,b) => println!("r:{}, g:{}, b:{}",r,g,b),
            Message::Quit => std::process::exit(0)
        }
    }
    
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
