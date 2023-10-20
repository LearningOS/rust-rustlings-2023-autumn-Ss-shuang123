<!--
 * @Author: ss
 * @Date: 2023-10-10 19:12:24
 * @LastEditTime: 2023-10-20 11:38:21
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

字符串字面值(字符切片)： \
不可变 \
& 一般是切片 \
.trim() 默认切片 \
+后面是切片

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

关联函数: \
类似于构造函数
```rust
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
```

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
impl<T> Point<T>{
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

# 2023-10-12
泛型Trait： \
trait是一种类型，用于将方法签名组合起来的方法集合。\
能方法重载 \
如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中\
```rust

// trait 相当于是声明一些方法
pub trait Summary { //pub：公开
    fn summarize(&self) -> String;
}

pub struct  Post{
    pub title: String,
    pub author: String,
    pub content: String
}
impl Summary for Post { // for 指向
    fn summarize(&self) -> String {
        format!("文章{}, 作者{}", self.title, self.author)
    }
}

fn main(){
    let post = Post{
        title:"你好".to_string(),
        author:"ss".to_string(),
        content:"ss".to_string()
    };
    println!("{}",post.summarize());
}
```

```rust
//特征约束
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//等价于
fn notify<T:Summary>(item:&T){
    println!("Breaking news! {}", item.summarize());
}

//多个特征约束
fn notify(item:&impl Summary+Other){}
fn notify<T:Summary+Other>(item:&T){}

//where约束
fn notify<T,U>(t:&T,u:&U)
where T:Summary+Other,
       U:Summary+Other
{
    //...
}

```
类似于重载特征
```rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T:Add<T,Output = T>>{
    x:T,
    y:T
}

impl <T:Add<T,Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p:Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

fn add<T:Add<T, Output = T>>(a:T, b:T) -> T{
    a + b
}
fn main() {
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}
```
Copy特征：\
任何基本类型的组合可以Copy, \
还有不可变引用&T

derive特征：\
#[derive(Debug)] 

特征对象：\
解决对象类型不同，但需要一个统一类型来处理 \
Box<dyn _> 任何实现_特征的类型都可以放其中 \

特征对象限制: \
使用特征对象时，Rust 必须使用动态分发 \
特征的所有方法满足以下条件 
1. 方法的返回类型不能是Self
2. 方法没有任何泛型参数

Vector: \
读取元素：\
```rust
&v[2];
v.get(2); //Option类型
```

HashMap: \
```rust
use std::collections::HashMap; //需要导入
let mut map = HashMap::new(); //创建
map.insert("a", 1); //插入
map.get("a"); //获取 返回Option
map.remove("a"); //删除
map.entry("a").or_insert(1); //是否有 无则插入 返回&mut
```
生命周期: \
指定引用的有效范围 \
引用的生命周期必须小于等于变量的生命周期 \
生命周期语法标注 函数名加上<'a> 变量名后面通常加上:&'a \
返回值等于参数中生命周期最短的那个 \
'a: 'b，是生命周期约束语法，表示'a的生命周期必须大于等于'b的生命周期 \
&'static 生命周期整个程序活得一样久

生命周期消除：\
返回值是一个引用,必定是一个参数的引用 \
所有可以省略标注

生命周期规则：\
1. 每一个引用都有其生命周期
2. 只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
3. 如果有多个输入生命周期参数，但其中一个是&self或&mut self，那么self的生命周期被赋予所有输出生命周期参数


# 2023-10-13
Crate包：\
独立的可编译单元 \
Package项目：\
一个库Package、二进制Packge(一般可单独运行) \
Package 是一个项目工程，而Crate包只是一个编译单元 \

mod模块：
pub关键字可见 默认情况下，所有的类型都是私有化的，包括函数、方法、结构体、枚举、常量，\ 
就连模块本身也是私有化的 父模块完全无法访问子模块中的私有项，\ 
但是子模块却可以访问父模块、父父..模块的私有项\
super: 父模块开始的引用 \
crate：包根 \
self：自身模块

use: \
与python类似 {}代替from import\
有as重命名 \
修改Cargo.toml：   \
在[dependencies] 区域添加：rand 

panic: \
不可恢复错误 \
俩种终止流程：栈展开和直接终止 \
Result类型.parse().unwrap() ： 解析成功 返回Ok，失败panic \


?操作符 \
传播错误 \
```rust
f.read_to_string(&mut s)?;

//等价于
let mut f = match f {
    // 打开文件成功，将file句柄赋值给f
    Ok(file) => file,
    // 打开文件失败，将错误返回(向上传播)
    Err(e) => return Err(e),
};
```

还能自动进行类型提升（转换） \
链式传播 \
处理错误值能返回，正确不能直接返回 \


# 2023-10-14
继续做题，相当于是复习的过程 \

# 2023-10-15
注释分为三个主要类型：代码注释、文档注释、包和模块注释 \
代码注释：// /* */ \
文档注释：/// /** */ 可利用cargo doc \
包和模块注释：//! #![crate_type = "lib"] \

格式化输出：\
常用println!标准输出和format!格式化文本 \
eprintln!标准错误输出 \
```rust
println!("hello,{}", "world");
println!("{value}", value=4);      // => "4" 带名称的参数必须放在不带名称参数的后面
format!("{:?}", (3, 4));          // => "(3, 4)"
```
[:#?] [:?] 用于输出无直接Display特征 \
格式化参数：5填充 <>^对齐 .精度 #进制 eE指数 \

目前基本上过完基础篇了； \



# 2023-10-16
继续阅读圣经，进入高级阶段了呜呼~ \
闭包：可以传递给变量、函数并且可以允许捕获调用者作用域的值 \
```rust
let action = || {
    println!("Hello, world!");
};
action();
//显示标注
let sum = |x:i32,y:i32| -> i32{
    x+y
}
```
能自动捕获作用域中的值  \
三种fn特征：Fn：不可变借用、FnMut：可变借用、FnOnce：所有权 \
TODO

```rust
fn fn_once<F>(func: F)   //func是闭包函数变量
where
    F: FnOnce(usize) -> bool + Copy, //代表转移所有权
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()}) //|z|{z == x.len()}闭包
}
```
FnMut是推导出的特征类型，与

迭代器： \
有next方法 所有权转移 返回option类型\

Iterator是迭代器特征，实现了它才能称为迭代器，才能调用 next \
实现IntoIterator特征 可以通过into_iter、iter、iter_mut方法转换为迭代器 \

消费者适配器：转移所有权 例如sum\
迭代适配器：返回一个新的迭代器，惰性初始化 zip map filter\
zip:把多个迭代器合并一个，map:对每个元素进行转换，filter:过滤掉不满足条件的元素 \

# 2023-10-18
栈拷贝一般是深拷贝 \
Box<T> 智能指针使用场景： \
1. 数据要储存在堆上
2. 将动态类型变为固定大小
3. 特征对象 Box<dyn name> \
Box<T>是一个智能指针，它指向堆上的数据，实现了Deref和Drop特征，可以自动解引用 \

Deref特征： \
实现了deref特征的类型，可以通过*解引用 \
```rust
impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Target{
        &self.0
    }
}
```
Deref能连续隐式解引用 \
三种Deref规则   \

Drop特征：  \
drop方法借用目标的可变引用  \

Cell： \
Cell类型针对的是实现Copy特征的值类型
RefCell：\
用于可变、不可变应用共存导致的问题 \
RC<T> Arc<T>： \
RC:指向底层数据的不可变的引用，仅仅复制了智能指针并增加了引用计数 \
Arc：原子化的Rc<T>智能指针，多线程 可以安全的在线程间共享 \
Rc 和 RefCell 又可以变了 \ 
多线程： \
move：转移所有权 \
线程结束：线程的代码执行完，线程就会自动结束,线程执行不完，一种是类似循环IO读取 一种是线程是循环 \
barrier：多个线程都执行到某个点后，才继续一起往后执行
局部变量：\
thread_local库 \
条件变量：控制线程的挂起和执行 std::sync::Condvar \
std::sync::Once 只被调用一次 \





# 2023-10-19
消息传递： \
多发送者,但接受者 std::sync::mpsc \
创建（异步）：std::sync::mpsc::channel \
发送：send
接收：recv、try_recv \
for循环接收 \
同步创建：std::sync::mpsc::sync_channel, send参数是消息缓存 \

Mutex：互斥锁 std::sync::Mutex ：\
在使用数据前必须先获取锁 \
在数据使用完成后，必须及时的释放锁 \
RwLock: 读写锁 std::sync::RwLock ：\
允许多个读，只允许一个写，读写不能同时存在 \
通常需要对读到的资源进行"长时间"的操作 \

信号量：控制当前正在运行的任务最大数量 \

原子类型： \


Macros： 宏编程 \
vec!宏： \
```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

类型转化： \
as：强制类型转换 数值型类型\
try_into 转化 \ 
AsRef 特性用于实现将某种类型转换为另一种类型的引用，而不是进行实际的拷贝或转移数据 \
AsMut 是Rust中的一个trait，用于表示类型可以被视为可变引用的类型。以将一个不可变引用转换为一个可变引用，然后对目标类型进行修改

# 2023-10-20
unsafe： \
裸指针： let address = 0x012345usize; \
