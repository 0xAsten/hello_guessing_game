use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // for i in v {
        //     println!("spawn: {}", i);
        //     thread::sleep(Duration::from_millis(1));
        // }

        println!("{:?}", v);
    });

    // println!("{:?}", v);

    for i in 1..5 {
        println!("main: {}", i);
    }

    handle.join().unwrap();

    channel();
    locking();
}

// using messeage to passe data between threads
// Channel: transmitter/receiver
use std::sync::mpsc;

fn channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("thread")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }

    // let received = rx.recv().unwrap();
    // println!("{}", received);
}

// shared state
// use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn locking() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", counter);
}
