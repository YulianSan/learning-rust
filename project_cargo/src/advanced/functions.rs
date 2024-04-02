pub fn example1() {
    fn add_one(n: i32) -> i32 {
        n + 1
    }

    // fn type accept functions and closures
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    println!("{}", do_twice(add_one, 5));
    println!("{}", do_twice(|n: i32| n + 1, 5));
}

pub fn example2() {
    let list_of_numbers = vec![32, 3, 54, 65, 1, 34, 43];
    let list_of_strings_1 = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    // or
    let list_of_strings_2: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // or
    // equal js use a function and the params is pass as args
    let list_of_strings_3: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

pub fn example3() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    // equal js
    let list_of_statuses = (1u32..20).map(Status::Value).collect::<Vec<Status>>();

    println!("{list_of_statuses:?}");
}

pub fn example4() {
    fn returns_closure_1() -> fn(i32) -> i32 {
        |n: i32| n + 1
    }

    fn returns_closure_2() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|n: i32| n + 1)
    }

    fn returns_closure_3() -> impl Fn(i32) -> i32 {
        |n: i32| n + 1
    }

    let closure_1 = returns_closure_1();
    let closure_2 = returns_closure_1();

    println!("{}", closure_1(10));
    println!("{}", closure_2(2));
}
