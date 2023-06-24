use crate::card::{cards, Suit};

#[derive(Default)]
pub struct CardComparators {
    pub natural_suit_order_comparator: NaturalSuitOrderComparator,
    pub unique_id_comparator: UniqueIdComparator,
    pub real_power_comparator: RealPowerComparator,
    pub raw_power_comparator: RawPowerComparator,
}

pub trait CardComparator {
    fn compare(&self, prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32;
    fn compare_no_prime(&self, id1: u32, id2: u32) -> i32 {
        self.compare(None, id1, id2)
    }
}
pub struct CombimedComparator {
    comparator_vec: Vec<Box<dyn CardComparator>>,
}
impl CardComparator for CombimedComparator {
    fn compare(&self, prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
        for com in self.comparator_vec.iter() {
            let result = com.compare(prime_suit, id1, id2);
            if result != 0 {
                return result;
            }
        }
        0
    }
}

#[derive(Default)]
pub struct UniqueIdComparator;
impl CardComparator for UniqueIdComparator {
    fn compare(&self, _prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
        id1 as i32 - id2 as i32
    }
}
#[derive(Default)]
pub struct NaturalSuitOrderComparator;
impl CardComparator for NaturalSuitOrderComparator {
    fn compare(&self, _prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
        let card1 = cards().by_id(id1);
        let card2 = cards().by_id(id2);
        // ignore Joker because never happens
        if card1.is_joker() || card2.is_joker() {
            return 0;
        }
        card1.suit.unwrap() as i32 - card2.suit.unwrap() as i32
    }
}
#[derive(Default)]
pub struct RawPowerComparator;
impl CardComparator for RawPowerComparator {
    fn compare(&self, _prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
        cards().by_id(id1).raw_power - cards().by_id(id2).raw_power
    }
}
#[derive(Default)]
pub struct RealPowerComparator;
impl CardComparator for RealPowerComparator {
    fn compare(&self, prime_suit: Option<Suit>, id1: u32, id2: u32) -> i32 {
        cards().by_id(id1).real_power(prime_suit) - cards().by_id(id2).real_power(prime_suit)
    }
}
