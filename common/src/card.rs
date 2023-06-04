use enum_iterator::{all, Sequence};
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

impl Suit {
    pub fn color(&self) -> Color {
        return match self {
            Suit::DIAMONDS => Color::RED,
            Suit::CLUBS => Color::BLACK,
            Suit::HEARTS => Color::RED,
            Suit::SPADES => Color::BLACK,
        };
    }
}

pub struct Cards {
    pub cards: [Card; 54],
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
        cards_vec.push(Card::of_black_joker());
        cards_vec.push(Card::of_red_joker());
        let cards: [Card; 54] = cards_vec.try_into().unwrap();
        Self { cards }
    }
}

impl Cards {
    pub fn by_id(&self, id: u32) -> &Card {
        &self.cards[(id as usize) % 54]
    }
}

#[derive(Debug)]
pub struct Card {
    pub intrinsic_id: u32,
    pub suit: Option<Suit>,
    pub color: Color,
    pub score: u32,
    pub is_flexible_prime: bool,
    pub raw_power: u32,
    pub numeric_card_num: Option<u32>,
}

impl Card {
    fn of_numeric_num_and_suit(suit: Suit, numeric_card_num: u32, intrinsic_id: u32) -> Self {
        Self {
            intrinsic_id,
            suit: Some(suit),
            color: suit.color(),
            score: score_of_numeric_number(numeric_card_num),
            is_flexible_prime: is_flexible_prime_of_numeric_number(numeric_card_num),
            raw_power: raw_power_of_numeric_number(numeric_card_num),
            numeric_card_num: Some(numeric_card_num),
        }
    }

    fn of_red_joker() -> Self {
        Self {
            intrinsic_id: 53,
            suit: None,
            color: Color::RED,
            score: 0,
            is_flexible_prime: true,
            raw_power: 16,
            numeric_card_num: None,
        }
    }

    fn of_black_joker() -> Self {
        Self {
            intrinsic_id: 52,
            suit: None,
            color: Color::BLACK,
            score: 0,
            is_flexible_prime: true,
            raw_power: 15,
            numeric_card_num: None,
        }
    }
}

fn score_of_numeric_number(num: u32) -> u32 {
    if num % 5 == 0 {
        return num;
    }
    if num == 13 {
        return 10;
    }
    return 0;
}

fn is_flexible_prime_of_numeric_number(num: u32) -> bool {
    num == 2 || num == 3 || num == 5
}

fn raw_power_of_numeric_number(num: u32) -> u32 {
    return if num == 1 { 14 } else { num };
}

pub struct PrimeOrSub {
    pub prime_suit: Option<Suit>,
    sub_suit: Option<Suit>,
}

impl PrimeOrSub {
    pub fn of_prime_suit(prime_suit: Option<Suit>) -> Self {
        Self {
            prime_suit,
            sub_suit: None,
        }
    }

    pub fn of_sub_suit(sub_suit: Suit) -> Self {
        Self {
            prime_suit: None,
            sub_suit: Some(sub_suit),
        }
    }

    pub fn is_prime(&self) -> bool {
        return match self.sub_suit {
            Some(_) => false,
            None => true,
        };
    }

    pub fn is_sub(&self) -> bool {
        !self.is_prime()
    }
}
