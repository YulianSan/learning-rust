pub fn mut_example() {
    let mut x: String = "hello".to_string();
    println!("The value of x is: {}", x);
    x += " there";
    println!("The value of x is: {}", x);
}

pub fn const_example() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}

pub fn scope_example() {
    let x = 5;

    {
        let x = x + 1;
        let x = x * 2;
        println!("inside block x = {}", x);
    }

    println!("outside block x = {}", x);
}

pub fn reassignment_example() {
    let spaces = "   ";

    // not allowed: spaces = spaces.len();
    let spaces = spaces.len();

    println!("{}", spaces);
}

pub fn type_example() {
    let x = 2.0;
    let y: f32 = 3.0;
    let letter: char = '😻';
    let tup = (500, 6.4, 1);
    let (.., z) = tup;
    let month: [&str; 3] = ["January", "February", "March"];
    let a = [3; 5];

    println!("x: {}, y: {}, z: {}, letter: {}", x, y, z, letter as i8);
    println!("tup: {:?}", tup);
    println!("month: {:?}", month);
    println!("a: {:?}", a);
}
