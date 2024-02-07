pub fn example1() {
    struct Point<T> {
        x: T,
        y: T,
    }

    struct ManyTypesGeneric<T, U, D> {
        x: T,
        y: U,
        z: D,
    }
    let x: [char; 4] = ['o', 'a', 'b', 'f'];
    let y: [i32; 4] = [12, 34, 2, 29];

    // error because T only can is a type
    // let point_error: Point<i32> = Point { x: 10.0, y: 21 };
    let point_right: Point<i32> = Point { x: 10, y: 21 };
    let many_types: ManyTypesGeneric<i32, char, String> = ManyTypesGeneric {
        x: 18,
        y: 'M',
        z: String::from("Yulian"),
    };

    // largest(&x);
    // largest(&y);
}

pub fn example2() {
    // using in enum
    enum Test<T> {
        Testing(T),
        Finish(),
    }

    let testing: Test<i32> = Test::Testing(32);
    let finish: Test<i32> = Test::Finish();
}

pub fn example3() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // using generics with impl
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }
    }

    let point = Point::new(1, 2);
    println!("{:?}", point);
}

pub fn example4() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // set type generic with impl with struct
    impl Point<i32> {
        fn new(x: i32, y: i32) -> Point<i32> {
            Point { x, y }
        }
    }

    // // error
    // let point = Point::new('o', 'a');

    let point = Point::new(1, 2);
    println!("{:?}", point);
}

pub fn example5() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let point1 = Point { x: 12, y: 'o' };
    let point2 = Point {
        x: String::from("test"),
        y: 12.32,
    };

    // // error because types no match
    // point1.mixup::<i32, i32>(point2);

    // types match
    // point is lost because ownership change
    let new_point = point1.mixup::<String, f32>(point2);
    println!("new point: {:?}", new_point);
}

// pub fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &value in list.iter() {
//         if value > largest {
//             largest = value;
//         }
//     }
//
//     largest
// }
