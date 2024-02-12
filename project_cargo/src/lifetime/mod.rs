pub fn example1() {
    let string1 = String::from("abcd");
    let string2 = "zjzxyz";

    let result = longest(&string1, string2);
    println!("The longest string is {}, string1: {}", result, string1);
}

pub fn example2() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    // get first sentence
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i.part: {}", i.part);
}
// pub fn example1_with_error() {
//     let string1 = String::from("abcd");
//     let result: &str;
//
//     {
//         // string2 has lifetime shorter than result, so it causes error
//         let string2 = String::from("zjzxyz");
//         result = longest(&string1, &string2);
//     }
//
//     println!("The longest string is {}", result);
// }

// says that the return is valid while the parameters exists
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
// // error because lifetime is not specified
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
