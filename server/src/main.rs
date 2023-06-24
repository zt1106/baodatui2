#![feature(async_closure)]
#![allow(unused_imports)]
#![allow(unused)]
pub mod handler;
pub mod rsocket;

use baodatui_common::card::cards;

// TODO should separate server and backend, so backend can be used directly for RL training

#[tokio::main]
async fn main() {
    dbg!(cards().by_id(1));
    println!("Hello, world!");
}
