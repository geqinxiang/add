use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];


    let handle=thread::spawn(|| {
        for i in 1..10 {
            println!("hi   number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();//使用 join的线程未执行完之前不执行下面程序
    let handle2=thread::spawn(|| {
        for i in 1..20 {
            println!("hi   number {} from the spawned22222 thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi   number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();//等待使用 join的线程 结束 方可结束程序

    let handle3 = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });//用move关键字是害怕线程在结束前会,v会被析构
    // drop(v); //在这里会报错handle3结束前不能被析构
    handle3.join().unwrap();
    // drop(v); // ok
}