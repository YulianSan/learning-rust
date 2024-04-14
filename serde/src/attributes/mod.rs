use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct S {
    f: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "et")]
enum E {
    #[serde(rename = "at")]
    A(String),
}

pub fn example1() {
    let e = E::A("test".to_string());
    let e_serialized = serde_json::to_string(&e).unwrap();
    let e_2 = serde_json::from_str::<E>(&e_serialized).unwrap();
    println!("{e_serialized}");
    println!("{e_2:?}");
}
