use std::{thread, time::Duration};

pub async fn example_simple_function() {
    thread::sleep(Duration::from_secs(1));
    println!("Hello, world async!");
}
