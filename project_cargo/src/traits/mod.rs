use std::fmt::Display;

trait Test {
    // default fn in trait
    fn test_old(&self) {
        println!("test");
    }
}

trait Build {
    fn test(&self) {}
}

struct TestStruct {
    x: i32,
    y: i32,
}

struct TestStruct2 {
    size: i32,
}

impl Test for TestStruct {}

impl Build for TestStruct {
    fn test(&self) {
        println!("x: {}", self.x);
    }
}

impl Build for TestStruct2 {
    fn test(&self) {
        println!("size: {}", self.size);
    }
}

pub fn example1() {
    let s1 = TestStruct { x: 10, y: 10 };
    let s2 = TestStruct2 { size: 20 };

    s1.test();
    s2.test();
}

pub fn example2() {
    let s1 = TestStruct { x: 10, y: 10 };
    let s2 = TestStruct { x: 20, y: 20 };

    // s1 is moved to need_impl
    // because no use &s1
    need_impl1(&s1);
    need_impls1(&s1);
    need_impls3(&s1, &s2);
}

pub fn example3() {
    #[derive(Debug)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // only impl if is Display and PartialOrd
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(10, 20);
    let pair2 = Pair::new(None::<i32>, None::<i32>);
    pair.cmp_display();
    // // pair2 no impl Display + PartialOrd
    // pair2.cmp_display();
    println!("pair2: {:?}", pair2);
}

pub fn example4() {}

fn need_impl1(x: &impl Build) {
    x.test();
}

// the same as need_impl1
fn need_impl2<T: Build>(x: T) {
    x.test();
}

// multi impl`s
fn need_impls1(x: &(impl Build + Test)) {
    x.test();
    x.test_old();
}

// multi impl`s with generic
fn need_impls2<T: Build + Test>(x: T) {
    x.test();
    x.test_old();
}

// multi impl`s with where
// better to read
fn need_impls3<T, J>(x: &T, y: &J)
where
    T: Build + Test,
    J: Build + Test,
{
    x.test();
    y.test();
    x.test_old();
    y.test_old();
}
