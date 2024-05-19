#![feature(print_internals)]
#![feature(fmt_helpers_for_derive)]

use serde::{Serialize, Deserialize};
use ciborium::{from_reader, into_writer};

mod packed;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let point = Point { x: 1f64, y: 2f64 };

    // // ciborium
    // let mut buffer = Vec::new();
    // into_writer(&point, &mut buffer).unwrap();
    // let decoded: Point = from_reader(buffer.as_slice()).unwrap();
    // println!("{:?}", decoded);

    // packed
    let mut serializer = packed::Serializer::new();
    point.serialize(&mut serializer).expect("Failed to serialize");
    let buffer = serializer.into_inner();
    println!("{:?}", buffer);
}
