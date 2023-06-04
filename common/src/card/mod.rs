use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
#[derive(Debug, Copy, Clone, Sequence)]
pub enum Color {
    RED,
    BLACK,
}
#[derive(Debug, Sequence, Copy, Clone)]
pub enum Suit {
    DIAMONDS,
    CLUBS,
    HEARTS,
    SPADES,
}

struct Cards {
    cards: [Card; 54],
}

impl Default for Cards {
    fn default() -> Self {
        let mut cards_vec: Vec<Card> = vec![];
        let mut intrinsic_id = 0;
        for suit in all::<Suit>() {
            for numeric_num in 1..14 {
                cards_vec.push(Card::of_numeric_num_and_suit(
                    suit,
                    numeric_num,
                    intrinsic_id,
                ));
                intrinsic_id += 1;
            }
        }
        let cards: [Card; 54] = cards_vec.try_into().unwrap();
        Self { cards }
    }
}

#[derive(Debug)]
struct Card {
    intrinsic_id: u32,
    suit: Option<Suit>,
    color: Color,
    score: u32,
    is_flexible_prime: bool,
    raw_power: u32,
    numeric_card_num: Option<u32>,
}

impl Card {
    fn of_numeric_num_and_suit(suit: Suit, numeric_num: u32, intrinsic_id: u32) -> Self {
        todo!()
    }

    fn of_red_joker() -> Self {
        todo!()
    }

    fn of_black_joker() -> Self {
        todo!()
    }
}
