/*
 * @Author: ss
 * @Date: 2023-10-11 21:59:31
 * @LastEditTime: 2023-10-11 21:59:31
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\enums\enums1.rs
 */
// enums1.rs
//
// No hints this time! ;)
 

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
