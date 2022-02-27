fn main() {
    let c1 = PokerCard{
       suit: PokerSuit::Clubs,
       value: 1
    };
    let c2 = PokerCard{
        suit: PokerSuit::Diamonds,
        value: 12
    };
}

struct PokerCard{
    suit: PokerSuit,
    value: u8,
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}