/*
 * @Author: ss
 * @Date: 2023-10-19 18:18:47
 * @LastEditTime: 2023-10-19 18:18:47
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\clippy\clippy2.rs
 */
// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.
 

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
