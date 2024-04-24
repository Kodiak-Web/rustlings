// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit = 0, 
    Echo = 1,
    Move = 2,
    ChangeColor = 3

}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
