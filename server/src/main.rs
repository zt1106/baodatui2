pub mod concurrent;
pub mod model;

use baodatui_common::card::cards;

fn main() {
    dbg!(cards().by_id(1));
    println!("Hello, world!");
}
