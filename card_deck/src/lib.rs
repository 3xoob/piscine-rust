use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

fn get_random(min: u8, max: u8) -> u8 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seed = now.subsec_nanos() as u64;
    min + (seed % (max - min + 1) as u64) as u8
}

impl Suit {
    pub fn random() -> Suit {
        let random_value = get_random(1, 4);
        Suit::translate(random_value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_value = get_random(1, 13);
        Rank::translate(random_value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    matches!((card.suit, card.rank), (Suit::Spade, Rank::Ace))
}
