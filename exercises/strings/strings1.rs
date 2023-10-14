/*
 * @Author: ss
 * @Date: 2023-10-12 13:15:20
 * @LastEditTime: 2023-10-12 13:15:20
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\strings\strings1.rs
 */
// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.
 

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
