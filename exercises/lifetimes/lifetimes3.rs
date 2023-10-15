/*
 * @Author: ss
 * @Date: 2023-10-15 19:49:33
 * @LastEditTime: 2023-10-15 19:49:33
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\lifetimes\lifetimes3.rs
 */
// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.
 

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
