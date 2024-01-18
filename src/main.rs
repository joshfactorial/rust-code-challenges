#[allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut hand_value: usize = 0;
        let mut aces_seen = 0;

        for card in &self.cards {
            hand_value += match *card {
                Card::King | Card::Queen | Card::Jack => 10,
                Card::Nine => 9,
                Card::Eight => 8,
                Card::Seven => 7,
                Card::Six => 6,
                Card::Five => 5,
                Card::Four => 4,
                Card::Three => 3,
                Card::Two => 2,
                Card::Ace => {
                    aces_seen += 1;
                    11
                },
            }
        }

        if aces_seen > 0 {
            // We have an ace and we are over 21, so the ace switches value from 11 to 1
            if hand_value > 21 {
                hand_value - (10 * aces_seen)
            // otherwise the ace stays an 11
            } else {
                hand_value
            }
        // No ace, so you get what you get
        } else {
            hand_value
        }
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}


#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 22);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Ace);
    hand.add(Card::Five);
    
    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 23);
}
