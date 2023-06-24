#![allow(unused_imports)]
#![allow(unused)]
use baodatui_common::{
    card::{cards, Card, Suit},
    comparator::{CardComparator, CardComparators, NaturalSuitOrderComparator},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn compare_demo(prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
    let c = NaturalSuitOrderComparator;
    c.compare(prime_suit, id1, id2)
}

#[wasm_bindgen]
pub fn get_card_by_id(id: u32) -> *const Card {
    cards().by_id(id) as *const Card
}
