pub fn example1() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter);

    println!("{:?}", &v1_iter.next());
    println!("{:?}", &v1_iter.next());

    // output: Got: 3
    for val in v1_iter.clone() {
        println!("Got: {}", val);
    }

    v1_iter.clone().sum::<i32>();
}

pub fn example2() {
    let v1 = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
}

pub fn example3() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let x: Vec<i32> = v1.into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>();
    println!("{:?}", x);
}
