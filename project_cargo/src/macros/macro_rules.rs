pub fn examples() {
    example3();
}

pub fn example1() {
    macro_rules! printhello {
        () => {
            println!("hello");
        };
    }

    printhello!();
}

pub fn example2() {
    macro_rules! times_five {
        ($n:expr) => {
            5 * $n
        };
    }

    println!("{}", times_five!(5 + 2));
}

pub fn example3() {
    macro_rules! vec_int {
        ($($i:expr),*) => {
            {
                let mut vec_int: Vec<i32> = Vec::new();
                $(
                    vec_int.push($i)
                // omg ºOº, this work
                );*;

                vec_int
            }
        }
    }

    let x = vec_int!(1, 2);
}
