#![feature(async_closure)]
pub mod container;
pub mod handler;
pub mod model;
pub mod rsocket;

use baodatui_common::card::cards;

#[tokio::main]
async fn main() {
    dbg!(cards().by_id(1));
    println!("Hello, world!");
}
