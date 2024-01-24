//!模式
//
///例子
///模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。
/// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）.函数参数、let 语句和 for 循环只能接受不可反驳的模式，因为当值不匹配时，程序无法进行有意义的操作。
///对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）if let 和 while let 表达式可以接受可反驳和不可反驳的模式，但编译器会对不可反驳的模式发出警告，
/// 因为根据定义它们旨在处理可能的失败：
///
fn main() {
    /* match分支 match是穷尽的表达式
    match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}*/
    let x:Option<u32>=Some(4);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }.expect("REASON");
    /*

if let 条件表达式 可以不穷尽
     */
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
/*
while let 条件循环
一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。
 */
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
//pop 方法取出 vector 的最后一个元素并返回 Some(value)
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    /*
    for 循环
    模式是 for 关键字直接跟随的值，正如 for x in y 中的 x
     */
    let v = vec!['a', 'b', 'c'];
//这里使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    /*
    let 语句
    let 变量赋值：
     let PATTERN = EXPRESSION;
     所以例如 let x = 5; 的情况，x 是一个代表 “将匹配到的值绑定到变量 x” 的模式。同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
     */
    let x = 5;
//Rust 会比较值 (1, 2, 3) 与模式 (x, y, z) 并发现此值匹配这个模式。
    let (x, y, z) = (1, 2, 3);
// let (x, y) = (1, 2, 3); 一个错误的模式结构，其中变量的数量不符合元组中元素的数量
//     为了修复这个错误，可以使用 _ 或 .. 来忽略元组中一个或多个值，
let (x, y,.. ) = (1, 2, 3);

    /*
    函数参数
     */
    foo(4);

    let Some(x) = Some(8)else { todo!() };
    println!("x:{}",x);
    if let x = 5 {
        println!("x:{}", x);
    };
    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
//字面值匹配
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // 匹配命名变量
    let x = Some(5);
    let y = 10;
//第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为我们在 match 表达式的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 y。
    match x {
        Some(50)=> println!("Got 50"),
        Some(y)=>println!("Matched {}",y),
        _=>println!("Default case, x = {:?}", x),
    }
//     多个模式 使用 | 语法匹配多个模式，它代表 或（or）运算符模式。
    let mut  x = 1;
    x=7;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
//     通过 ..= 匹配值的范围
    x = 5;
    // x=6;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    // 解构并分解值
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a:{}",a);
    println!("b:{}",b);
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("x:{}",x);
    println!("y:{}",y);
    let p = Point { x:5, y: 7 };
    match p {
        Point {x:0,y:0}=>{println!("x=0 and y=0")},
        Point{x:0,y}=>{println!("x=0")},
        Point{x,y:0}=>{println!("y=0")},
        Point{x,y }=>{println!("x={},y={}",x,y)},
    }
    //结构枚举
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Move {x, y}=>{println!("Move in the x direction {} and in the y direction { }",x,y)},
        Message::Write(s)=>{println!("String::{}",s)},
        Message::ChangeColor(x,y,z)=>{println!("Change the color to red { }, green { }, and blue { }",x,y,z)},
        Message::Quit=>{}

    }
//     解构嵌套的结构体和枚举
    let col=Color::Hsv(4,5,66);
    let col2=Color::Rgb(42,52,66);
    let msg=Message2::ChangeColor(col);
    match msg {
        Message2::Move {x, y}=>{println!("Move in the x direction {} and in the y direction { }",x,y)},
        Message2::Write(s)=>{println!("String::{}",s)},
        Message2::ChangeColor(col)=>{match col {
            Color::Rgb(x,y,z)=>{println!("  Change the color to Rgb::red { }, green { }, and blue { }",x,y,z)},
            Color::Hsv(x,y,z)=>{println!("  Change the color to Hsv::red { }, green { }, and blue { }",x,y,z)}
        }}
        _ =>{}
    }
    let msg2=Message2::ChangeColor(col2);
    match msg2 {
        Message2::ChangeColor(Color::Hsv(x,y,z))=>{println!("  Change the color to Hsv::red { }, green { }, and blue { }",x,y,z)},
        Message2::ChangeColor(Color::Rgb(x,y,z))=>{println!("  Change the color to Rgb::red { }, green { }, and blue { }",x,y,z)},
        _=>{}
    }
    //结构元组和结构体
    let ((a,b),Point{x,y})=(("aaa".to_string(),5.2),Point{x:44,y:55});
    // 忽略模式中的值
    foo1(2,1 );

    foo2(2,1 );
    // 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
    // 忽略元组中的部分值
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
    //通过在名字前以一个 _ 开头来忽略未使用的变量
    // 这里得到了警告说未使用变量 y，不过没有警告说使用 _x。

    let _x = 5;
    let y = 10;
//     只使用 _ 和使用以下划线开头的名称有些微妙的不同：比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s);//这里会报错，因为Some(_s) = s会将s的 值move 给_s
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);//这里不会报错，因为Some(_) = s会将s的 值move 给_
// 用 .. 忽略剩余值
    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }
    let orr=(1,2,4,2,4,1,3);
    match orr {
        (frist,..,end)=>{println!("Some numbers: {}, {}",frist,end)},

    }
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),//这里的y是上边定义的Y   let y = 10;
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    let x = 4;
    let y = false;
    // 这个匹配条件表明此分支值匹配 x 值为 4、5 或 6 同时 y 为 true 的情况
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    // @ 绑定
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
//x 部分就是一个模式！类似于之前对 let 所做的，可以在函数参数中匹配元组
fn foo( x: i32)->i32 {
    // code goes here

    x+1
}
fn foo1(_:i32,x: i32)->i32 {
    // code goes here
    println!("{}",&x);
    x+1
}

fn foo2(x: i32,_:i32)->i32 {
    // code goes here
    println!("{}",&x);
    x+1
}
// 想做忽略多个值的方法没有成功
// fn foo3(x: i32,.. ..:i32)->i32 {
//     // code goes here
//     println!("{}",&x);
//     x+1
// }

//foo此模块的值命名空间中仅定义一次
// fn foo(x: i32,y:i32) ->i32{
//     x+y
// }
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
struct Point {
    x: i32,
    y: i32,
}
struct Point2 {
    x: i32,
    y: i32,
    z:i32
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Message3 {
    Hello { id: i32 },
}