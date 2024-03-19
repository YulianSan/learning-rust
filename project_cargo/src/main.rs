#![allow(unused)]

// mod iterators;
mod oop;

fn main() {
    oop::gui::example1();
    // game_test::main();
    // iterators::example3();
}

// mod traits;
// mod collections;
// mod macros;
// mod errors;
// use crate::collections::vec;
// mod game;

// fn main() {
// traits::example1();
// macros::example1();
// vec::example1();
// errors::example2();
// errors::example1();
// println!("file: {}", errors::example7("test.txt").expect("error out"));
// game::game2();
// }
// mod modules;
// crate import from current mod or files
// create a alias
// use crate::modules::{
//     food::{show_food, Food},
//     vegetables::{show_vegetable as sv, Asparagus},
// };

// import std::io and std::io::Write
// use std::io::{self, Write};

// // can import all modules in std
// use std::*;

// mod game;
// mod variables;
// mod functions;
// mod flow;
// mod ownership;
// mod slices;
// mod structs;
// mod impls;

// fn main() {
// modules::test();
// game::game()
// variables::mut_example();
// variables::const_example();
// variables::scope_example();
// variables::reassignment_example();
// variables::type_example();
// println!("{}", functions::plus_one(1));
// functions::block();
// functions::flow();
// flow::loops();
// flow::loop_loop_break();
// flow::fors();
// ownership::example();
// slices::example3();
// structs::example6();
// enums::example5();
// consume_modules();
// }

// fn consume_modules() {
//     let asparagus = Asparagus {
//         name: String::from("Asparagus"),
//         color: String::from("Green"),
//     };
//
//     let food = Food {
//         name: String::from("Food"),
//         color: String::from("Red"),
//     };
//
//     sv(asparagus);
//     show_food(food);
// }
