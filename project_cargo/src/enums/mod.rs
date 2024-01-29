#![allow(dead_code)]
use std::net::Ipv4Addr;

pub fn example1() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4 = 4,
        V6 = 6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let job = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", home);
    println!("{:?}", job);
    println!("number: {:?}", IpAddrKind::V4 as i32);
    println!("string: {:?}", IpAddrKind::V4);
}

pub fn example2() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let job = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", job);
    println!("{:?}", String::from("127.0.0.1").parse::<Ipv4Addr>());
}

pub fn example3() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let _m = Message::Write(String::from("hello"));
    let _m1 = Message::Move { x: 1, y: 2 };
}

pub fn example4() {
    let some_number = Some(5);
    let _some_string = Some("a string");

    // Option is a possible null value
    let absent_number: Option<i32> = None;

    // no make this
    // let _sum = some_number + 2;
    // use match instead
    match some_number {
        Some(i) => println!("match: {}", i + 2),
        None => println!("none in string"),
    }
    // or use if let
    if let Some(i) = some_number {
        println!("if let: {}", i + 2);
    }

    match absent_number {
        Some(i) => println!("{}", i),
        None => println!("none in absent"),
    }

    if let Some(i) = absent_number {
        println!("Never join here {}", i);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usize),
}

pub fn example5() {
    let coin1 = Coin::Quarter(2);
    let coin2 = Coin::Penny;
    let coin3 = Coin::Dime;

    check_coin(coin1);
    check_coin(coin2);
    check_coin(coin3);
}

fn check_coin(coin: Coin) {
    match coin {
        Coin::Quarter(x) => println!("Lucky quarter {:?}", x),
        Coin::Penny => println!("Penny without value"),
        _ => println!("Not a quarter"),
    }
}
