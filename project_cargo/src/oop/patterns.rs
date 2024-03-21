pub fn example1() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn example2() {
    // get only the two first numbers
    let (x, y, ..) = (1, 2, 3, 4);
    println!("{x}, {y}");
}

pub fn example3() {
    // destruct
    fn coord(&(x, y): &(i32, i32)) {
        println!("coord {x}, {y}");
    }

    let test = (10, 3);

    coord(&test);
    coord(&test);
}
