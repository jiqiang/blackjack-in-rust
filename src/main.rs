extern crate rand;

use rand::Rng;

const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

#[derive(Debug, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: String,
}

struct Game {
    cards: Vec<Card>,
}

impl Game {
    pub fn new(num_of_decks: u16, num_of_shuffles: u16, cut_at: usize) -> Self {
        let mut cards: Vec<Card> = vec![];
        Self::add_decks(&mut cards, num_of_decks);
        Self::shuffle_cards(&mut cards, num_of_shuffles);
        Self::cut_cards(&mut cards, cut_at);
        Game { cards }
    }

    fn add_decks(cards: &mut Vec<Card>, num_of_decks: u16) {
        let mut idx: u16 = 0;
        while idx < num_of_decks {
            for &rank in RANKS.iter() {
                for suit in SUITS.iter() {
                    cards.push(Card {
                        suit: suit.clone(),
                        rank: rank.to_string(),
                    });
                }
            }
            idx += 1;
        }
    }

    fn shuffle_cards(cards: &mut Vec<Card>, num_of_shuffles: u16) {
        let mut i: u16 = 0;
        while i < num_of_shuffles {
            let mut m = cards.len();
            let mut rng = rand::thread_rng();
            while m > 1 {
                m -= 1;
                let r = rng.gen_range(0, m);
                cards.swap(r, m);
            }
            i += 1;
        }
    }

    fn cut_cards(cards: &mut Vec<Card>, middle: usize) {
        let mut bottom = cards.split_off(middle);
        bottom.append(cards);
        cards.append(&mut bottom);
    }

    pub fn issue_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            cards: vec![],
        }
    }

    pub fn take_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn hand_value(&self) -> u16 {
        let mut sum_without_aces: u16 = 0;
        let mut num_of_aces: u16 = 0;

        for card in self.cards.iter() {
            match card.rank.as_str() {
                "A" => num_of_aces += 1,
                "10" | "J" | "Q" | "K" => sum_without_aces += 10,
                "2" => sum_without_aces += 2,
                "3" => sum_without_aces += 3,
                "4" => sum_without_aces += 4,
                "5" => sum_without_aces += 5,
                "6" => sum_without_aces += 6,
                "7" => sum_without_aces += 7,
                "8" => sum_without_aces += 8,
                "9" => sum_without_aces += 9,
                _ => panic!("unknow rank"),
            }
        }

        if num_of_aces == 0 {
            return sum_without_aces;
        }

        let sum_with_small_aces = sum_without_aces + num_of_aces;

        // must check sum_with_small_aces > 21 otherwise 21 - sum_with_small_aces will panic if the result type is not a u16
        if sum_with_small_aces > 21 || 21 - sum_with_small_aces < 10 {
            return sum_with_small_aces;
        }

        sum_with_small_aces + 10
    }
}

fn main() {
    let mut game = Game::new(3, 5, 22);

    let mut dealer = Player::new("Dealer");
    let mut player = Player::new("Glenn");

    let mut dealer_stand = false;
    let mut player_stand = false;

    loop {
        if !dealer_stand {
            if dealer.hand_value() >= 17 {
                dealer_stand = true;
            } else if let Some(card) = game.issue_card() {
                dealer.take_card(card);
            } else {
                panic!("out of cards");
            }
        }

        if !player_stand {
            if player.hand_value() >= 17 {
                player_stand = true;
            } else if let Some(card) = game.issue_card() {
                player.take_card(card);
            } else {
                panic!("out of cards");
            }
        }

        if dealer_stand && player_stand {
            break;
        }
    }

    println!("{:?}", dealer);
    println!("{}", dealer.hand_value());
    println!("{:?}", player);
    println!("{}", player.hand_value());
}
