pub fn example() {
    let s = String::from("hello");
    let s1: String = s;
    // s is invalid here
    // let x = return_pointer_string();
    // x is possible null because ownership

    // o can make this: println!("{}", (*x));
    // but the pointer is valid
    // println!("{:?}", x);
    //

    takes_ownership(s1);
    // s1 is invalid here

    let mut s3 = String::from("hello2");
    let s4: &mut String = &mut s3;
    // s3 is valid, because s4 is a pointer
    (*s4).push_str(" world");
    // s3 is changed to "hello2 world"

    let (s2, len) = calculate_length(s3);
    // s3 is invalid here

    let new_len = calculate_length2(&s2);
    // s2 is valid here

    println!("The length of '{}' is {}-{}.", s2, len, new_len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    (s.clone(), s.len())
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

// this is invalid
// fn return_pointer_string() -> &String {
//     let x = String::from("Hello");
//     &x
// }
