/*
 * @Author: ss
 * @Date: 2023-10-10 21:35:25
 * @LastEditTime: 2023-10-10 21:35:25
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\functions\functions5.rs
 */
// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
