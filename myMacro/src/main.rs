//!宏（Macro）
//!使用 macro_rules! 的 声明（Declarative）宏，和三种 过程（Procedural）宏：
//!
//!1、自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
//!2、类属性（Attribute-like）宏定义可用于任意项的自定义属性
//!3、类函数宏看起来像函数不过作用于作为参数传递的 token
//!
//! 宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）
//!
//! 宏扮演了函数扮演的角色。但宏有一些函数所没有的附加能力。
//! 1、一个函数签名必须声明函数参数个数和类型。相比之下，宏能够接收不同数量的参数：
//! 2、宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait。而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现。
//! 3、实现宏不如实现函数的一面是宏定义要比函数定义更复杂，因为你正在编写生成 Rust 代码的 Rust 代码。由于这样的间接性，宏定义通常要比函数定义更难阅读、理解以及维护。
//! 4、宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用。


fn main() {
    println!("Hello, world!");
    let mut v=vec1![1,2,3];
    v.push(3);
    println!("{:?}",v);
    Pancakes::hello_macro();
}
///接着使用 macro_rules! 和宏名称开始宏定义，且所定义的宏并 不带 感叹号。名字后跟大括号表示宏定义体，在该例中宏名称是 vec
#[macro_export]
macro_rules! vec1 {
    // 模式所匹配的是 Rust 代码结构而不是值
    // 此处有一个分支模式 ( $( $x:expr ),* ) ，后跟 => 以及和模式相关的代码块。
    // 这里这个宏只有一个模式，那就只有一个有效匹配方向，其他任何模式方向（译者注：不匹配这个模式）都会导致错误
    // 使用美元符号（$）在宏系统中声明一个变量来包含匹配该模式的 Rust 代码。美元符号明确表明这是一个宏变量而不是普通 Rust 变量。
    // 之后是一对括号，其捕获了符合括号内模式的值用以在替代代码中使用。$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式命名为 $x。
    // 紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
    // expr翻译表达式
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

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

pub trait HelloMacro {
    fn hello_macro();
}