/*
 * @Author: ss
 * @Date: 2023-10-14 15:15:17
 * @LastEditTime: 2023-10-14 15:25:19
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\options\options3.rs
 */
// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.
 

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<&Point> = Some(&Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
