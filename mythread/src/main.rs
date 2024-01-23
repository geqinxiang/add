use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicUsize,Ordering};
use std::{hint, thread};
use std::ops::Deref;
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
    //线程间的消息传递
    // 使用 mpsc::channel 函数创建一个新的信道；mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
    let (tx, rx) = mpsc::channel();
    // let (tx1, rx1) = mpsc::channel();
    thread::spawn(move || {
        // tx1.send( String::from("hi======")).unwrap();
        let val = String::from("hi----");
        tx.send(val).unwrap();
        //下面这段代码对线程间的消息传递没有意义，只是想看两个线程是并行的
        for i in 1..5 {
            println!("hi   number {} from the spawned333333333 thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

    });
    // let rec=rx1.try_recv().unwrap();
    // println!("Got: {}", rec);
    let received = rx.recv().unwrap();
    println!("Got11111: {}", received);
    //线程间的消息传递
    //实现多生产者 一个消费者，通过指针克隆
    let (tx2, rx2) = mpsc::channel();
    let tx21 = tx2.clone();//在创建新线程之前，我们对发送者调用了 clone 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("22more"),
            String::from("22messages"),
            String::from("22for"),
            String::from("22you"),
        ];

        for val in vals {
            tx21.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got======: {}", received);
    }
//     线程间消息共享 多线程和多所有权
//     通过 Arc实现多所有权
// Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型
//     Arc::clone 重新定义的是指针，直指指向的区域没有变化 通过此实现多所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("--------::{}::{}",i,&num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());



}