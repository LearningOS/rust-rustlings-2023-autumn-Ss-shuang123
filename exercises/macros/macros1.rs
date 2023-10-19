/*
 * @Author: ss
 * @Date: 2023-10-19 17:54:13
 * @LastEditTime: 2023-10-19 17:54:13
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\macros\macros1.rs
 */
// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.
 

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
