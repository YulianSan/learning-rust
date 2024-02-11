use std::{collections::HashMap, env::temp_dir};

pub fn example1() {
    let mut v: HashMap<&str, i32> = HashMap::new();

    v.insert("test", 20);
    v.insert("test1", 0);
    // create if not exists
    let x1 = v.entry("test").or_insert(10).clone();
    let x2 = v.entry("test2").or_insert(10).clone();

    println!("hash: {:?}", v);
    println!("x1: {:?}, x2: {:?}", x1, x2);
}

pub fn example2() {
    // convert vec to hashMap
    let teams = vec![String::from("Blue"), String::from("red")];
    let scores = vec![10, 20];

    let teams_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("{:?}", teams_scores);

    // for in hashmap without & is moved the ownership
    for (key, value) in &teams_scores {
        println!("key: {}, value: {}", key, value);
    }

    println!("{:?}", teams_scores);
}

pub fn example3() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // counts how many times each word appears
    for word in text.split(' ') {
        // start with 0
        // is exists get current value in hashmap
        let count = map.entry(word).or_insert(0);
        // increment
        *count += 1;
        println!("{}, count: {}", word, count);
    }

    println!("{:?}", map);
}

pub fn example4() {
    let mut v = HashMap::new();

    v.insert(String::from("sum"), |value: i32, value2: i32| {
        value + value2
    });
    // v.insert(String::from("sub"), |value: i32, value2: i32| {
    //     value + value2
    // });

    let x = match v.get("sum") {
        Some(f) => f,
        None => panic!("error"),
    };

    println!("sum: {}", x(1, 2));
}

pub fn example5() {
    let mut v = HashMap::new();
    v.insert(String::from("blue"), 1);
    v.insert(String::from("red"), 1);

    let x = String::from("purple");
    // get value or default value if not exists
    let y = v.get(&x).copied().unwrap_or(0);
    println!("y: {}", y);
}
