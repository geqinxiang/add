//!高级类型
//! 首先我们从一个关于为什么 newtype 与类型一样有用的更宽泛的讨论开始。
//! 接着会转向类型别名（type aliases），一个类似于 newtype 但有着稍微不同的语义的功能。
//! 我们还会讨论 ! 类型和动态大小类型。
use std::collections::HashMap;
use std::{fmt, io};
use std::cmp::Ordering;
use std::io::Error;

type ge1= Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;
fn main() {
    println!("Hello, world!");
    let mut p=people( HashMap::new());
    p.insert("tom".to_string());
    p.insert("jason".to_string());
    p.insert("tom".to_string());
    p.insert("jason".to_string());
    println!("{:?}",p);
//     类型别名用来创建类型同义词
//      ge是 i32 的 同义词
    type  ge=i32;
    let mut  i:ge=9;
    let mut j:i32=8;
    let mut k=&i+&j;
    println!("k={i}+{j}={}",k);
//     ge1是

    let g1:ge1=Box::new( ||println!("geqx"));
    takes_long_type(g1);

    bar();
    // let mut  pa:String=String::new();
    //从不返回的 never type
    let mut i=9;
    loop {
        println!("请输入");
        let mut  pa:String=String::new();//将pa定义到loop上面，下面的io::stdin().read_line(）不能添加到pa里，不知道是为什么
        io::stdin().read_line(&mut pa).expect("error");
        println!("pa1={}",pa);
        let pa: i32 = match pa.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,//continue 的值是 !。也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值。
        };
        println!("pa2={}",pa);
        match  pa.cmp(&i){
            Ordering::Less=>println!("太小"),
            Ordering::Greater=>println!("太大"),
            Ordering::Equal=> {
                println!("ok");
                break;
            }
        }
    }
//     最后一个有着 ! 类型的表达式是 loop：
    print!("forever ");
    // 这里，循环永远也不结束，所以此表达式的值是 !。但是如果引入 break 这就不为真了，因为循环在执行到 break 后就会终止。
    let mut n=0;
    loop {
        print!("and ever ");
        n=n+1;
        if n==100{
            break;
        }
    }
//     动态大小类型和 Sized trait
// Rust 需要知道有关类型的某些细节，例如为特定类型的值需要分配多少空间。
// 因为str是动态大小类型 ，所以下面代码是错误的。
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
//   虽然 &T 是一个储存了 T 所在的内存位置的单个值，&str 则是 两个 值：str 的地址和其长度。
//     这里是 Rust 中动态大小类型的常规用法：它们有一些额外的元信息来储存动态信息的大小。
// 编译时就知道&str变量的大小。所以下面代码是正确的
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
    println!("{}",s1);
    println!("{}",s2);
//     另一个动态大小类型：trait
//     我们提到了为了将 trait 用于 trait 对象，必须将它们放入指针之后，比如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait> 也可以）
// Rust 隐式的为每一个泛型函数增加了 Sized bound。
    // 对于如下泛型函数定义：
    //
    // fn generic<T>(t: T) {
    //     // --snip--
    // }
    // 实际上被当作如下处理：
    //
    //
    // fn generic<T: Sized>(t: T) {
    //     // --snip--
    // }
}

///为了类型安全和抽象而使用 newtype 模式
///
#[derive(Debug)]
struct people(
    HashMap<i32, String>
);

impl people {
    fn insert(&mut self,name:String){
        let mut  i=self.getmaxkey();
        i+=1;
        self.0.insert(i,name);
    }
    fn getmaxkey(&self)->i32{
        let mut max=0;
        for key in self.0.keys() {
             if key> &max {
                 max= *key;
             }
        }
        max

    }

}

fn takes_long_type(f:ge1) {
    // --snip--
}

fn returns_long_type() -> ge1 {
    // --snip--
    Box::new(|| ())
}
// 没有用别名前
pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, Error>;
    fn flush(&mut self) -> std::result::Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> std::result::Result<(), Error>;
}
/// 使用别名后
///type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
///从不返回的 never type
/// 这读 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数（diverging functions）
///
fn bar()   {
    println!("hi nihao");

}
//never type 的另一个用途是 panic!
// 下面是Option的unwrap方法
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }