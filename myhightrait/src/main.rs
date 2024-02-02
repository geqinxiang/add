use std::fmt;
use std::ops::Add;

fn main() {
    // 关联类型在 trait 定义中指定占位符类型
    println!("Hello, world!");
    let mut  c=Counter::new();
    for _i in [1,2,3,4,5,6,7,8,9,10] {
        c.next();
        println!("{}",c.count);
    }

    // 默认泛型类型参数和运算符重载
    let p=Point { x: 3, y: 3 };
    println!("{:?}={:?}",
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        p
    );
    let mm=Millimeters(6);
    let m=Meters(4);
    let mm1=mm+m;
    println!(" + ={:?}",  mm1);
    //     完全限定语法与消歧义：调用相同名称的方法

    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);
    // 完全限定语法定义为：<Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}",  <Dog as Animal>::baby_name(  ));

//     父 trait 用于在另一个 trait 中使用某 trait 的功能
    let p = Point { x: 1, y: 3 };
    p.outline_print();
//     newtype 模式用以在外部类型上实现外部 trait
    let mut  w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    let e=w.pop();
    println!("w = {}", w);
    w.push(String::from(",GQX"));
    println!("w = {}", w);
    let mut w2=Wrapper::f3(10);
    println!("{:?}len:{},capacity:{}",w2.0,w2.len(),w2.capacity());
    w2.push("a".to_string());
    w2.push("b".to_string());
    w2.push("c".to_string());
    w2.push("d".to_string());
    w2.push("e".to_string());
    w2.push("f".to_string());
    w2.push("h".to_string());
    w2.push("i".to_string());
    w2.push("j".to_string());
    w2.push("k".to_string());
    w2.push("l".to_string());
    w2.push("m".to_string());
    w2.push("n".to_string());
    w2.push("o".to_string());
    w2.push("p".to_string());
    w2.push("q".to_string());
    w2.push("r".to_string());
    println!("w2:{}",w2);
    println!("{:?}len:{},capacity:{}",w2.0,w2.len(),w2.capacity());
}
///关联类型在 trait 定义中指定占位符类型
/// 一个带有关联类型的 trait 的例子是标准库提供的 Iterator trait。
/// 它有一个叫做 Item 的关联类型来替代遍历的值的类型。
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
pub trait Iterator2<T> {
    fn next2(&mut self) -> Option<T>;
}
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 9 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
// impl<T> Iterator2<T> for Counter {
//
//
//     fn next2(&mut self) -> Option<T> {
//         // --snip--
//         if self.count < 15 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
///默认泛型类型参数和运算符重载
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
///重载+运算符
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
///这是一个带有一个方法和一个关联类型的 trait。
/// 比较陌生的部分是尖括号中的 Rhs=Self：这个语法叫做 默认类型参数（default type parameters）。
/// Rhs 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。
/// 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。
// trait Add<Rhs=Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
/// 完全限定语法与消歧义：调用相同名称的方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
/// 父 trait 用于在另一个 trait 中使用某 trait 的功能
///
/// 有时我们可能会需要编写一个依赖另一个 trait 的 trait 定义：
/// 对于一个实现了第一个 trait 的类型，你希望要求这个类型也实现了第二个 trait。
/// 如此就可使 trait 定义使用第二个 trait 的关联项。
/// 这个所需的 trait 是我们实现的 trait 的 父（超）trait（supertrait）。
///
/// 指定了 OutlinePrint 需要 Display trait，
/// 在 outline_print 中使用 to_string，其会为任何实现 Display 的类型自动实现。

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // 在 outline_print 中使用 to_string，其会为任何实现 Display 的类型自动实现。
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
///newtype 模式用以在外部类型上实现外部 trait
/// 因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 Vec<T> 的所有方法，这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper。
struct Wrapper(Vec<String>);

impl Wrapper {
    fn len(&self)->usize{
        self.0.len()
    }
    fn pop(&mut self) ->Option<String>{
        self.0.pop()
    }
    fn push(&mut self,s:String){
        self.0.push(s);
    }
    fn f3(x:usize) -> Wrapper {
        Wrapper(Vec::with_capacity(x))
    }
    fn capacity(&self)->usize{
        self.0.capacity()
    }
}
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 来访问其内部的 Vec<T>
        write!(f, "[{}]", self.0.join(", "))
    }
}
