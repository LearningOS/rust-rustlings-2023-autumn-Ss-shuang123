/*
 * @Author: ss
 * @Date: 2023-10-19 17:58:55
 * @LastEditTime: 2023-10-19 18:07:19
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\macros\macros3.rs
 */
// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.
 

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
      my_macro!();
}
