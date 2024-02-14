#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// this say that this only run in 'cargo test'
#[cfg(test)]
mod tests {
    // import all alias in module parent to this module
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

#[cfg(test)]
mod tests2 {
    fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, add_two(2));
    }
}

#[cfg(test)]
mod test3 {
    fn return_panic() -> ! {
        panic!("error test");
    }

    #[test]
    fn test_custom_message() {
        let name: String = String::from("Carol, San");
        // the second parameter is a custom message, work like println passing {}
        assert!(name.contains("San"), "name should contain '{}'", "San");
    }

    #[test]
    // expected message contain "no match message" if not test not pass
    #[should_panic(expected = "no match message")]
    // ignore a test
    #[ignore = "test"]
    fn test_should_panic() {
        return_panic();
    }

    #[test]
    fn test_using_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("error".to_string())
        }
    }
}
