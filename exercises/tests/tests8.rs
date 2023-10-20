/*
 * @Author: ss
 * @Date: 2023-10-20 12:27:28
 * @LastEditTime: 2023-10-20 12:27:28
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\tests\tests8.rs
 */
// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.
 

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
