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
