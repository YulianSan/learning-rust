pub fn example1() {
    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let x = my_vec!(1, 2, 3);
    let x: Vec<i32> = my_vec!();
    println!("{:?}", x);
}
