//!高级函数与闭包
//! 传递已经定义的函数而不是重新定义闭包作为参数,函数满足类型 fn,不要与闭包 trait 的 Fn 相混淆。fn 被称为 函数指针。通过函数指针允许我们使用函数作为另一个函数的参数。
use std::fmt;
use std::fmt::Pointer;
use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    // 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。
    let n=do_2(add_1,1);
    println!("{}",n);

    // 作为一个既可以使用内联定义的闭包又可以使用命名函数的例子，让我们看看一个 map 的应用。
    // 使用 map 函数将一个数字 vector 转换为一个字符串 vector，就可以使用闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| (i+1).to_string()).collect();

    // 或者可以将函数作为 map 的参数来代替闭包，像是这样：
    let list_of_numbers = vec![1, 2, 3];
    // 注意这里必须使用 “高级 trait” 部分讲到的完全限定语法，因为存在多个叫做 to_string 的函数；这里使用了定义于 ToString trait 的 to_string 函数
    let list_of_strings2: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}",list_of_strings);
    println!("{:?}",list_of_strings2);

//      “枚举值” 部分中定义的每一个枚举成员也变成了一个构造函数。我们可以使用这些构造函数作为实现了闭包 trait 的函数指针，这意味着可以指定构造函数作为接受闭包的方法的参数，
//    一些人倾向于函数风格，一些人喜欢闭包。 这两种形式最终都会产生同样的代码，所以请使用对你来说更明白的形式吧。
    let v:Vec<Status>  = (0u32..20).map(Status::Value).collect();
    let v2: Vec<Status> = (0u32..20).map(|x|Status::Value(x*2)).collect();
    println!("{:?}",v);
    println!("{:?}",v2);


//     返回闭包
//     闭包表现为 trait，这意味着不能直接返回闭包。
// 对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。
// 但是这不能用于闭包，因为它们没有一个可返回的具体类型；例如不允许使用函数指针 fn 作为返回值类型。
    let b=returns_closure();
    // let mut f: & fmt::Formatter<'_> = &();
    println!("返回闭包:{:?}",b(5));
}
fn add_1(i:i32)->i32{
    i+1
}
fn do_2(f:fn(i32)->i32,arg:i32)->i32{
    f(arg)+f(arg)
}
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// 返回闭包
//     闭包表现为 trait，这意味着不能直接返回闭包。
// 对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。
// 但是这不能用于闭包，因为它们没有一个可返回的具体类型；例如不允许使用函数指针 fn 作为返回值类型。

// 这段代码尝试直接返回闭包，它并不能编译：
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

