use std::{
    thread::{self, spawn, JoinHandle},
    time::Duration,
};
pub mod channel;

pub fn example1() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("thread 1, index {i}");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..10 {
        println!("example1, index {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // wait thread
    handle.join().unwrap();
}

pub fn example2() {
    let v = vec![1, 2, 3];
    let x = 10;

    // erro because x no is moved to thread
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", x);
    // });

    // move v to thread

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
