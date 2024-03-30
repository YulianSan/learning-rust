use std::fmt::{self, Display};
use std::ops::{Add, Deref};
use std::process::Output;

pub fn example1() {
    trait TestType {
        type T;
        fn test(&self) -> Self::T;
    }

    trait TestGeneric<T> {
        fn test(&self) -> T;
    }

    struct TestStruct2 {}
    struct TestStruct1<T> {
        value: T,
    }

    // only one type
    impl TestType for TestStruct2 {
        type T = String;

        fn test(&self) -> Self::T {
            println!("test type");
            String::from("test")
        }
    }

    // error when try implement different type
    // impl TestType for TestStruct2 {
    //     type T = i32;
    //
    //     fn test(&self) -> Self::T {
    //         println!("test type");
    //         10
    //     }
    // }

    // can use multiples types
    impl TestGeneric<String> for TestStruct1<String> {
        fn test(&self) -> String {
            println!("string {}", self.value);
            self.value.clone()
        }
    }

    impl TestGeneric<i32> for TestStruct1<i32> {
        fn test(&self) -> i32 {
            println!("i32 {}", self.value);
            self.value
        }
    }

    let test_generic_i32 = TestStruct1::<i32> { value: 10 };
    let test_generic_string = TestStruct1::<String> {
        value: String::from("test"),
    };
    let test_type = TestStruct2 {};

    test_generic_i32.test();
    test_generic_string.test();
    test_type.test();
}

pub fn example2() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // set default generic
    // trait Add<Rhs = Self> {
    //     type Output;
    //
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Self) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

pub fn example3() {
    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    // Add has default generic Rhs = Self
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Self::Output {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let me = Meters(1);
    let mi = Millimeters(1000);
    // Millimeters always in right because impl trait add
    println!("{:?}", mi + me);
    // this cause error
    // println!("{:?}", me + mi);
}

pub fn example4() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    // call fly method of impl Human
    person.fly();
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);
}

pub fn example5() {
    struct Dog;
    trait Animal {
        fn baby_name() -> String;
    }

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    // need say the struct name
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

pub fn example6() {
    // this trait require implement Display, force
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "test")
        }
    }

    impl OutlinePrint for Point {}

    let point = Point { x: 0, y: 10 };

    point.outline_print();
}

pub fn example7() {
    struct DisplayVec<T: Display>(Vec<T>);

    impl fmt::Display for DisplayVec<i32> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // write!(
            //     f,
            //     "{}",
            //     &self
            //         .0
            //         .iter()
            //         .map(|n| n.to_string())
            //         .collect::<Vec<String>>()
            //         .join(", ")
            // )

            write!(
                f,
                "numbers {}",
                &self.0.iter().fold(String::new(), |acc, n| {
                    if acc.is_empty() {
                        n.to_string()
                    } else {
                        acc + ", " + &n.to_string()
                    }
                })
            )
        }
    }

    // implement deref to allow use methods of Vec
    impl<T: Display> Deref for DisplayVec<T> {
        type Target = Vec<T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl fmt::Display for DisplayVec<String> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "strings {}", self.0.join(", "))
        }
    }

    let x = DisplayVec(vec![String::from("hello"), String::from("world")]);
    let y = DisplayVec(vec![1, 2, 123, 54, 90]);
    println!("x = {}", x);
    println!("y = {}", y);
}
