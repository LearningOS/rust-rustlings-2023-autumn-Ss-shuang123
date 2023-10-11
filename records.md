<!--
 * @Author: ss
 * @Date: 2023-10-10 19:12:24
 * @LastEditTime: 2023-10-11 21:31:26
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\records.md
-->
# 2023-10-9
学习了rust一些基本语法，根据[rust圣经 ](https://course.rs/about-book.html)学习到了2.3 所有权和借用 \
变量默认不可变，使用mut修饰可变 \
变量所有权: \
变量绑定有一个所有者，只有所有者才能修改变量的值，当所有者离开作用域，变量绑定将被丢弃. 


```rust
let s1 = String::from("hello");
let s2 = s1; //s1失效
println!("{}, world!", s1); //error
```
结构体复用：\
..struct_name \

数组切片： \
```rust
array = [1,2,3,4,5] 
&array[1..3] //左闭右开 
//基本类型在rust赋值是以Copy形式
```

引用借用: 
```rust
let x = 5;
let y = &x;
// y是x的引用 他们不相等

//可变引用与不可变引用不能同时存在
//不可多个可变引用
Non-Lexical Lifetimes(NLL)

```
```
```rust
let s = "hello";
{ // s无效
    let s = "hello, world"; // 新s有效
} // s无效
```
栈：\
函数局部变量、指向堆上的指针、函数参数、函数返回值 

# 2023-10-10 
本地vscode使用rustlings出现点问题，不能直接按照readme里面安装方式 \
安装好rust之后，在目录下使用 
```bash
cargo install --path=.
```
完成了15道题目

# 2023-10-11
继续做+看圣经
Option枚举：\
Option<T>是一个泛型枚举，它的成员是Some<T>和None，\
Some<T>中包含一个T类型的值，None不包含值。\
Option<T>类型用于编码一个值可能存在或不存在的情况，\

数组array:
栈上，连续存放，固定长度，不可变
```rust
let a = [1, 2, 3]; // a: [i32; 3]
let a = [3:5]; //[类型；长度] 某个值出现N次
```
数组元素非基本类型不能直接赋值copy
数组切片：
```rust
let a = [0, 1, 2, 3, 4];
let silece:&[i32] = &a[1..3]; //&[1,2]
```

if:
是表达式
```rust
if condition {
    // code here
} else if condition {
    // code here
} else {
    // code here
}
```
保证每个分支返回的类型一样(事实上，这种说法不完全准确)

for:
```rust
for i in 0..5 {
    println!("{}", i);
}
for item in array{
    println!("{}", item);
} //转移所有权,后续不能使用array
for item in &array {
    println!("{}", item);
} //不会改变array，后续可以使用array
for item in &mut array {
    *item += 1;
} //会改变array，后续可以使用array
```

break:
还可以返回值

loop：
循环 无参数搭配break

while：
需要一个条件控制循环
while通常要索引去访问数组，而for则不需要

match:
必须穷尽所有可能性
_ 通配符
if let匹配单个值
```
if let Some(3) = v {
    println!("three");
} // 变量遮蔽 
```

```rust
match value {
    pattern => expression,
    pattern => {
        expression;
    },
    _ => expression,
}
```
模式匹配：\
let,for和match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )


```rust
if let PATTERN = SOME_VALUE { //if let

}
while let Some(top) = stack.pop() { //while let
    println!("{}", top);
}
let Some(x) = some_option_value; //error 因为右边可能是None

```
option:
Some(T) 是 Rust 中的一个枚举变体，用于表示一个值存在的情况. \
Some(42)不等于42.
```rust
fn main() {
    let maybe_value: Option<i32> = Some(42);

    match maybe_value {
        Some(value) => {
            println!("The value is: {}", value);
            // 解构Some 这里是 42
        }
        None => {
            println!("No value");
        }
    }
}
```
方法Method: \
对象和方法定义是分离的

有些东西有点小蒙啊[TODO]

```rust
struct Point {
    x: f64,
    y: f64,
}
//关联函数
impl Point{
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}
//另一种写法
impl Point{
    fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }
}
```
泛型和特征：\
多态:
```rust
fn add<T:std::ops::Add<Output=T>>(a:T,b:T) -> T{
    a+b
}//不是所有 T 类型都能进行相加操作，所以我们需要对 T 进行约束
```
泛型：\
所有泛型参数都要在函数签名中声明
```rust
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//泛型结构体
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T{
    fn x(&self)->&T{
        &self.x
    }
}
//const泛型
//传引用是为了适应不同类型
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
//基于值的泛型参数 const泛型
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
```


