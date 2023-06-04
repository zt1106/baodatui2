use card::Cards;

pub mod card;

// static global data
#[derive(Default)]
pub struct CommonStatic {
    pub cards: Cards,
}
