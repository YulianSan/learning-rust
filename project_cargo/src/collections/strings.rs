pub fn example1() {
    // String can be changed
    let mut s = String::from("hello");

    // push string
    s.push_str(", world!");
    // push char
    s.push('!');

    println!("{}", s);
}

pub fn example2() {
    // String can be concatenated
    let h = String::from("hello");
    let mut w = String::from("world");
    let e = String::from("!");
    // h without clone is 'removed'
    // the fn add:
    // fn add(mut self, other: &str) -> String {
    //     self.push_str(other);
    //     // string is moved
    //     self
    // }
    let hw = h.clone() + " " + &w + &e;
    // or using format, this no moved h
    let hwf = format!("{h} {w}{e}");
    w.push_str(", again");

    println!("hw: {}", hw);
    println!("hw with format: {}", hw);
}

pub fn example3() {
    // length 3
    let v = String::from("Здр");
    // length in bytes is 6 because utf-8
    let b = v.as_bytes();
    let c = v.chars();
    println!("bytes: {:?}", b);
    println!("char: {:?}", c);
}
