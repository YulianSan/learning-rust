pub fn example() {
    let s = String::from("hello");
    let s1: String = s;
    // s is invalid here

    let s3 = String::from("hello2");
    let (s2, len) = calculate_length(s3);
    // s3 is invalid here
    println!("The length of '{}' is {}.", s2, len);
    takes_ownership(s1);
    // s1 is invalid here
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    (s.clone(), s.len())
}
