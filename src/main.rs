use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suites = ["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9","10", "Jack", "Queen", "King"];

        let mut cards = vec![];
        for suit in suites{
            for value in values{
                let card = format!("{} of {}", value, suit);
                cards.push(card)
            }
        }
        Deck{cards}
    }

    fn suffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);

    }
}

fn main() {
    let mut deck = Deck::new();

    deck.suffle();

    println!("heres your deck {:#?}", deck);
}
