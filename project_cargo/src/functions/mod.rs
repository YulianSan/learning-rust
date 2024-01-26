pub fn block() {
    let y = {
        let x: u8 = 3;
        // return x + 1 to be used;
        x + 1
    };

    println!("The value of y is: {}", y);
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
