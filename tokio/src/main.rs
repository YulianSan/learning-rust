#![warn(dead_code)]

use std::{thread, time::Duration};

mod examples;
mod mini;

#[tokio::main]
async fn main() {
    mini::cli().await;
}
// async fn main() {
//     // await to run
//     let op = examples::example_simple_function();
//
//     thread::sleep(Duration::from_secs(1));
//     println!("Hello, world!");
//
//     op.await;
// }
// async fn main() -> Result<()> {
//     match mini::main().await {
//         Ok(()) => {}
//         Err(err) => eprintln!("Error: {}", err),
//     };
//
//     Ok(())
// }
