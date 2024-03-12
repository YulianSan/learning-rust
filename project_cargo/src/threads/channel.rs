use std::{sync::mpsc, thread, time::Duration};

pub fn example1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("hi");
        tx.send(value).unwrap();
        // value is moved
        // println!("{value}");
    });

    // wait thread send message
    // try_recv no wait thread
    let recived = rx.recv().unwrap();
    println!("{recived}");
}

pub fn example2() {
    let (tx, rx) = mpsc::channel();
    // clone trans..
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("thread 1 message1"),
            String::from("thread 1 message2"),
            String::from("thread 1 message3"),
            String::from("thread 1 message4"),
            String::from("thread 1 message5"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("thread 2 message1"),
            String::from("thread 2 message2"),
            String::from("thread 2 message3"),
            String::from("thread 2 message4"),
            String::from("thread 2 message5"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // recived message from two threads
    for recived in rx {
        println!("message: {recived}");
    }
}
