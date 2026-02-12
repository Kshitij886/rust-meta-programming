#![allow(unused)]
use std::collections::HashMap;

macro_rules! vector {
    ( $( $x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )*
        temp_vec
        }
    }
}
// decelarative macros
macro_rules! mapify {
    ($($x:expr => $y:expr), *) => {{
        let mut temp_hashmap = std::collections::HashMap::new();
        $(
            temp_hashmap.insert($x,$y);
        )*
        temp_hashmap
    }};
}
fn main() {
    let vec = vector![1, 2, 3, 4];
    let map = mapify! {
        "a" => 1,
        "b" => 2,
        "c" => 3
    };
    println!("{:?}", vec);
    println!("{:?}", map);
}
