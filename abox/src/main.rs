use crate::List::Cons;
use crate::List::Nil;
fn main() {
    // println!("Hello, world!");
    // let b:Box<i32>=Box::new(5);
    // println!("{}",b);
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list:List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}",list);
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}