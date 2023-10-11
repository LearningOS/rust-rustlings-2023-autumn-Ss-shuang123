/*
 * @Author: ss
 * @Date: 2023-10-10 21:34:15
 * @LastEditTime: 2023-10-10 21:34:15
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\functions\functions3.rs
 */
// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
