use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn example1() {
    let point = Point { x: 10, y: 10 };
    let point_serialized = serde_json::to_string(&point).unwrap();
    let point_2 = serde_json::from_str::<Point>(&point_serialized).unwrap();
    println!("{point_serialized}");
    println!("{point_2:?}");
}
