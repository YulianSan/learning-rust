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
    enum Gender {
        Male,
        Female,
    }

    struct Person {
        name: String,
        age: i32,
        gender: Gender,
    }
    // destruct
    fn coord(&(x, y): &(i32, i32)) {
        println!("coord {x}, {y}");
    }

    // destruct, but is better use method instead of function
    fn show_name(
        Person {
            // rename param
            name: name_r,
            age,
            gender,
        }: &Person,
    ) {
        match gender {
            Gender::Male => print!("male, "),
            Gender::Female => print!("female, "),
        }
        println!("{name_r}, {age}");
    }

    let test = (10, 3);
    let person_test = Person {
        name: "test".to_string(),
        age: 10,
        gender: Gender::Male,
    };

    coord(&test);
    coord(&test);
    show_name(&person_test);
}

pub fn example4() {
    let x = (6, 2, 3);

    match x {
        // multiple patterns
        (1, ..) | (.., 1) => println!("1, .. or .., 1"),
        (2..=5, ..) => println!("first between 2 and 5, .."),
        (1, y, z) => println!("1, {y}, {z}"),
        (_, _, _) => println!("default"),
        (.., z) => println!("default, {z}"),
    }
}

pub fn example5() {
    let y = 'b';

    match y {
        'a'..='z' => println!("is a letter lowercase"),
        'A'..='Z' => println!("is a letter uppercase"),
        _ => println!("is't a letter"),
    }
}

pub fn example6() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

pub fn example7() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

pub fn example8() {
    let num = Some(4);

    match num {
        // adding other condition
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

pub fn example9() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // the [variable @ ...] get the value of variable or property
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
