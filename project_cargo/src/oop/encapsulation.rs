mod priva {
    pub struct Average {
        list: Vec<i32>,
        average: f64,
    }

    impl Average {
        pub fn new() -> Average {
            Average {
                list: vec![],
                average: 0.0,
            }
        }
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

pub fn example1() {
    use crate::oop::encapsulation::priva::Average;
    let mut x = Average::new();
    x.add(20);
    x.add(10);
    x.add(15);
    // x.list, x.update_average and x.average is private

    println!("average: {}", x.average());
    x.remove();
    x.remove();
    println!("average: {}", x.average());
}
