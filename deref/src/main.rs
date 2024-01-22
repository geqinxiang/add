//!智能指针
//
///
//
use std::ops::Deref;
use std::vec::Drain;
use std::fmt::Display;
use crate::List::{Cons, Nil};
use crate::List2::{Cons as Cons2, Nil as Nil2};
use crate::List3::{Cons as Cons3, Nil as Nil3};
use crate::List4::{Cons as Cons4, Nil as Nil4};
use std::rc::Rc;//引用计数器智能指针
use std::cell::RefCell;
fn main() {
    let x=5;//这个5存放在盏上
    let y=Box::new(x);//用智能指针，copy 5 放在堆上
    println!("{}",x);
    println!("{}",y);//不允许比较数字的引用与数字，因为它们是不同的类型。必须使用解引用来实现 解引用符*
    println!("{}",*y);
    println!("{}",x);
    let m:Mybox<String>=Mybox::new("DDDDD".to_string());
    hello(&m);//内部隐式类型转换 &Mybox<String>->&Sring->&str
    let t1=&m;
    let t2=t1.deref();
    let t3=t2.deref();
    {
        let m:Mybox<String>=Mybox::new("FFFFFFF".to_string());
    }
    // let n=&m.0;//这个有和没有都不影响m析构在最后
    // std::mem::drop(m);//显示释放智能指针的内存
    let m:Mybox<String>=Mybox::new("EEEEE".to_string());//m虽然改变了，但是第一个m的值并没有drop(),不管有用还是没有用，还在内存中存，只在方法的最后才能被释放

    let m2:Mybox<String>=Mybox::new("55555".to_string());
    let a=List2::Cons(1,Box::new(List2::Cons(2, Box::new(List2::Cons(3,Box::new( List2::Nil) )))));
    println!("{:?}",a);
    let b=List2::Cons(4,Box::new(a));
    println!("{:?}",b);
    // let c=List2::Cons(5,Box::new(a));//会报错a已经move
    // println!("{:?}",a);
    let a=Rc::new(List::Cons(1,Rc::new(Cons(10,Rc::new(Nil)))));
    let a1=Rc::new(List::Cons(2,Rc::new(Cons(20,Rc::new(Nil)))));
    let b=Cons(5,Rc::clone(&a));//并不是真的克隆技术
    println!("-----:{:?}",b);
    println!("-----:{:?}",a);
    println!("-----:{:?}",a1);
    let c=Cons(6,Rc::clone(&a));
    println!("{:?}",c);
    println!("{:?}",Rc::strong_count(&a));
    {
        let d = Cons(7, Rc::clone(&a));
        println!("{:?}", d);
        println!("{:?}",Rc::strong_count(&a));
    }
    println!("{:?}",a);
    println!("{:?}",Rc::strong_count(&a));
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    limit_tracker.set_value(100);
    limit_tracker.set_value(8);
    println!("--------------{:?}",mock_messenger.sent_messages);
    limit_tracker.set_value(90);
    println!("--------------{:?}",mock_messenger.sent_messages);
    limit_tracker.set_value(9);
    println!("--------------{:?}",mock_messenger.sent_messages);
    limit_tracker.set_value(85);
    println!("{}--------------{:?}",limit_tracker.value,mock_messenger.sent_messages);

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List3::Cons(Rc::clone(&value),   Rc::new(  List3::Nil)));

    let b = List3::Cons(Rc::new(RefCell::new(3)),   Rc::clone(&a ));
    let c = List3::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    let l4r=Rc::new(List4::Cons(3,RefCell::new(Rc::new(List4::Cons(2,RefCell::new(Rc::new(List4::Nil)))))));
    println!("l4r initial rc count = {}", Rc::strong_count(&l4r));
    println!("l4r next item = {:?}", l4r.tail());
    let l4r1=Rc::new(Cons4(4,RefCell::new(Rc::clone(&l4r))));
    println!("l4r rc count after b creation = {}", Rc::strong_count(&l4r));
    println!("l4r1 initial rc count = {}", Rc::strong_count(&l4r1));
    println!("l4r1 next item = {:?}", l4r1.tail());
//
    if let Some(link) = l4r.tail() {
        *link.borrow_mut() = Rc::clone(&l4r1);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&l4r1));
    println!("a rc count after changing a = {}", Rc::strong_count(&l4r));
    // println!("-----------------b   = {:?}",  &l4r1 );//打印会有溢出报错
   // println!("----------------a     = {:?}",  &l4r );//打印会有溢出报错
    println!("a next item = {:?}", l4r.tail());
    println!("a   = {:?}", l4r );
}

fn  hello(name:&str){
    println!("Hello,{}",name);
}

struct Mybox<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> Mybox<T> {
    fn new(x: T) -> Mybox<T> {
        Mybox(x)
    }
}

impl <T: std::fmt::Display>Deref for Mybox<T> {
    type  Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

impl<T: std::fmt::Display> Drop for Mybox<T> {
    fn drop(&mut self) {
        println!("dropping custom with data {}",self.0)
    }
    
}
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
#[derive(Debug)]
enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}
#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}
impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {

        match self {
            List4::Cons(_, item) =>  Some(item),
            List4::Nil => None,
        }
    }
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}
// impl<String> Drop for String  {
//     fn drop(&mut self) {
//         println!("dropping custom with data {}",self.0)
//     }
//
// }
pub trait Messenger {
    fn send(& self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        println!("max:{},value:{}",&self.max,&self.value);
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
// use super::*;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages:RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // self.sent_messages.push(String::from(message));
        self.sent_messages.borrow_mut().push(String::from(message));

            // let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            //
            // one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));


    }
}


