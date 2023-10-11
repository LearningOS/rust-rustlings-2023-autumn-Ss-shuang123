/*
 * @Author: ss
 * @Date: 2023-10-10 21:34:01
 * @LastEditTime: 2023-10-10 21:34:01
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\functions\functions2.rs
 */
// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
