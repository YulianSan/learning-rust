pub fn example1() {
    // Vec of i32
    let v: Vec<i32> = Vec::from([1, 2, 3]);
    println!("{:?}", v);
    // Create a vector with macro
    // vector have type Vec<i32>
    let mut v1 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);

    let three = &v1[2];
    // same addr
    println!("three three: {:?}", std::ptr::addr_of!(*three));
    println!("three v1[2]: {:?}", std::ptr::addr_of!(v1[2]));

    if let Some(x) = v1.get(2) {
        println!("three v1[2] inside if: {}", x);
    }

    match v1.get(10) {
        Some(x) => println!("three v1[10]: {}", x),
        None => println!("three v1[10]: None"),
    }
}

pub fn example2() {
    let mut v = vec![1, 2, 3];
    // let first = &v[0]; cause panic because vec change and get new addr
    let first = v[0];

    v.push(4);

    println!("The first element is: {}", first);
}

pub fn example3() {
    let mut v = vec![1, 2, 3, 4, 5];

    // should pass by reference if not v ownership change
    for index in &mut v {
        *index *= 2;
        println!("{}", index);
    }

    println!("The first element is: {}", v[0]);
}
