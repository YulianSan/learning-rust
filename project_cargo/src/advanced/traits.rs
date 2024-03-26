use crate::advanced::traits;

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
