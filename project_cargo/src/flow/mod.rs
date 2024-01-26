pub fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // returing value in loop;
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn loop_loop_break() {
    let mut count = 0;

    'main_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'main_loop;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

pub fn fors() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..=4).collect::<Vec<i32>>() {
        println!("number: {number}");
    }
}
