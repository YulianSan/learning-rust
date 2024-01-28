#[allow(unused)]
pub fn example2() {
    let s = "hello world";
    first_word_index(&s.to_string());

    println!("s: {:?}", s.bytes());
    println!("s: {:?}", s.as_bytes());
}

#[allow(unused)]
pub fn example3() {
    let a = [1, 12, 20, 38, 21];
    let slice = &a[1..3];

    // println!("slice: {:?}", slice);
    assert_eq!(slice, &[12, 20]);
}

#[allow(unused)]
pub fn example() {
    let mut s = String::from("hello world");
    // let word = first_word_index(&s);
    // let first_word = &mut s[0..word as usize];
    // println!("first_word: {}", first_word);
    let word = first_word(&s);

    println!("word: {}, s: {}", word, s);
    s.clear();
}

#[allow(unused)]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    println!("bytes: {:?}", bytes.iter());
    println!("s[0]: {}", bytes[0] as char);

    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, item:{}", i, item);
        if item == b' ' {
            // [..=i] to get space together
            return &s[..i];
        }
    }

    &s
}

#[allow(unused)]
fn first_word_index(s: &String) -> i32 {
    let bytes = s.as_bytes();

    println!("bytes: {:?}", bytes.iter());
    println!("s[0]: {}", bytes[0] as char);

    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, item:{}", i, item);
        if item == b' ' {
            return i as i32;
        }
    }

    -1
}

