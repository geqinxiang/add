use std::{hint, thread};
use std::thread::JoinHandle;

//全局变量在 Rust 中被称为 静态（static）变量
// 访问不可变静态变量是安全的
static HELLO_WORLD: &str = "Hello, world!";
// 如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争
static mut COUNTER: u32 = 0;
// 常量与不可变静态变量的一个微妙的区别是静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。
// 另一方面，常量则允许在任何被用到的时候复制其数据。
// 另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main()  {
    // 解引用裸指针
    // 裸指针与引用和智能指针的区别在于
    // 1、允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
    // 2、不保证指向有效的内存
    // 3、允许为空
    //4、 不能实现任何自动清理功能
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        *r2=6;
        println!("r2 is: {}", *r2);
        println!("num is: {}", &num);
        println!("r3 is: {}", *r3);
        println!("r1 is: {}", *r1);
        *r3=7;
        println!("r3 is: {}", *r3);
        println!("num is: {}", &num);
    }
    //     调用不安全函数或方法
    unsafe {
        dangerous();
    }
    //创建不安全代码的安全抽象
    let mut ll=vec![1,2,3,4,5,6];
    let r=&mut ll[..];
    let (a,b)=spite_at_mut(r,3);
    println!("a:{:?}",a);
    println!("b:{:?}",b);
    let address = 0x01234usize;
    let r = address as *mut i32;
    //  slice::from_raw_parts_mut 这段代码获取任意内存地址并创建了一个长为一万的 slice：
    let values: &[i32] = unsafe { std::slice::from_raw_parts_mut(r, 10000) };
    // extern 块中声明的函数在 Rust 代码中总是不安全的。
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // 使用可变静态变量
    println!("name is: {}", HELLO_WORLD);
    //可变静态变量不会move,静态变量中的值有一个固定的内存地址,使用这个值总是会访问相同的地址
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);

    unsafe {
        println!("1COUNTER: {}", COUNTER);
    }
    let handle = thread::spawn(||
        {
            add_to_count(3);

            unsafe {
                println!("2COUNTER: {}", COUNTER);
            }
        });
    let handle2 = thread::spawn(||
        {
            add_to_count(3);

            unsafe {
                println!("5COUNTER: {}", COUNTER);
            }
        });
    let handle3 = thread::spawn(||
        {
            add_to_count(3);

            unsafe {
                println!("6COUNTER: {}", COUNTER);
            }
        });
    unsafe {
        println!("3COUNTER: {}", COUNTER);
    }
    handle.join().unwrap();
    unsafe {
        println!("4COUNTER: {}", COUNTER);
    }
    let stu1=stu{
        name:String::from("tom"),
        sex:String::from("N"),
        birthday:String::from("1995-10-01")
    };
    stu1.add_to_count(1);
    unsafe {
        println!("7COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {
    println!("ni hao unsafe")
}
fn spite_at_mut(  vs: &mut [i32], m:usize)->(&mut [i32],&mut [i32]){
    let n=vs.len();
    let ptr = vs.as_mut_ptr();
    assert!(m <=vs.len() );
    unsafe{
       (
           std::slice::from_raw_parts_mut(ptr,m),
           std::slice::from_raw_parts_mut(ptr.add(m), n - m),
       )
    }

}
// extern，有助于创建和使用 外部函数接口（Foreign Function Interface，FFI）。外部函数接口是一个编程语言用以定义函数的方式，
// 其允许不同（外部）编程语言调用这些函数。
// extern 块中声明的函数在 Rust 代码中总是不安全的。

extern "C" {
    fn abs(input: i32) -> i32;
}

/// 从其它语言调用 Rust 函数
/// 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。
/// 不同于创建整个 extern 块，就在 fn 关键字之前增加 extern 关键字并为相关函数指定所用到的 ABI。
/// 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
/// Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，不过会使其名称更难以阅读。
/// 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。
///
///下面函数一旦其编译为动态库并从 C 语言中链接，call_from_c 函数就能够在 C 代码中访问：
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait Foo {
    // methods go here
    fn add_to_count(&self,inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
}
struct stu{
    name:String,
    sex:String,
    birthday:String,

}
unsafe impl Foo for  stu {

}