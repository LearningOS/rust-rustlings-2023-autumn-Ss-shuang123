/*
 * @Author: ss
 * @Date: 2023-10-19 17:58:16
 * @LastEditTime: 2023-10-19 17:58:16
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\macros\macros2.rs
 */
// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.
 

fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
