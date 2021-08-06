use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
};

#[derive(PartialOrd, Ord, Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn next(self) -> Option<Self> {
        use Rank::*;
        match self {
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => Some(Nine),
            Nine => Some(Ten),
            Ten => Some(Jack),
            Jack => Some(Queen),
            Queen => Some(King),
            King => Some(Ace),
            Ace => Some(Two),
        }
    }
}
impl TryFrom<&str> for Rank {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Rank::*;
        match value {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "10" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            _ => Err("Invalid char. Should be one of 1-10,J,Q,K,A"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl TryFrom<char> for Suit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Suit::*;
        match value {
            'S' => Ok(Spade),
            'H' => Ok(Heart),
            'D' => Ok(Diamond),
            'C' => Ok(Club),
            _ => Err("Invalid char. Should be one of 1-10,J,Q,K,A"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len < 2 {
            return Err("String too short");
        }
        Ok(Card {
            rank: Rank::try_from(&value[..(len - 1)])?,
            suit: Suit::try_from(value.chars().last().unwrap())?,
        })
    }
}

impl Card {
    fn five_cards(hand: &str) -> Result<[Card; 5], &'static str> {
        let mut cards = vec![];
        for s in hand.split(' ') {
            let c = Card::try_from(s)?;
            cards.push(c);
        }
        if cards.len() != 5 {
            Err("Hand should contain 5 cards")
        } else {
            Ok([cards[0], cards[1], cards[2], cards[3], cards[4]])
        }
    }
}

#[derive(PartialOrd, Ord, Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Category {
    HighCard([Rank; 5]),
    OnePair([Rank; 4]),
    TwoPair([Rank; 3]),
    ThreeOfaKind([Rank; 3]),
    Straight(Rank),
    Flush([Rank; 5]),
    FullHouse([Rank; 2]),
    FourOfaKind([Rank; 2]),
    StraightFlush(Rank),
}

impl From<&[Card; 5]> for Category {
    fn from(cards: &[Card; 5]) -> Self {
        use Category::*;
        let mut cards = *cards;
        cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        let mut cards_by_suit = HashMap::new();
        let mut cards_by_rank = BTreeMap::new();
        cards.iter().for_each(|c| {
            cards_by_suit
                .entry(c.suit)
                .or_insert_with(Vec::<Card>::new)
                .push(*c);
            cards_by_rank
                .entry(c.rank)
                .or_insert_with(Vec::<Card>::new)
                .push(*c);
        });
        let mut cards_by_suit = cards_by_suit.into_values().collect::<Vec<_>>();
        cards_by_suit.sort_by_key(|a| a.len());
        let mut cards_by_rank = cards_by_rank.into_values().collect::<Vec<_>>();
        cards_by_rank.sort_by(|a, b| match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Equal => a.last().unwrap().rank.cmp(&b.last().unwrap().rank),
            x => x,
        });

        let is_straight_normal = cards
            .iter()
            .skip(1)
            .try_fold(cards[0], |prev, c| {
                if prev.rank.next() == Some(c.rank) {
                    Some(*c)
                } else {
                    None
                }
            })
            .is_some();
        let is_straight_ace_low = [cards[4], cards[0], cards[1], cards[2], cards[3]]
            .iter()
            .skip(1)
            .try_fold(cards[4], |prev, c| {
                if prev.rank.next() == Some(c.rank) {
                    Some(*c)
                } else {
                    None
                }
            })
            .is_some();
        let straight_high;
        if is_straight_normal {
            straight_high = *cards.last().unwrap();
        } else {
            straight_high = cards[3];
        }
        let is_straight = is_straight_normal || is_straight_ace_low;
        if is_straight && cards_by_suit.len() == 1 {
            StraightFlush(straight_high.rank)
        } else if cards_by_rank.last().unwrap().len() == 4 {
            FourOfaKind([cards_by_rank[1][0].rank, cards_by_rank[0][0].rank])
        } else if cards_by_rank.last().unwrap().len() == 3
            && cards_by_rank.first().unwrap().len() == 2
        {
            FullHouse([cards_by_rank[1][0].rank, cards_by_rank[0][0].rank])
        } else if cards_by_suit.len() == 1 {
            Flush([
                cards[4].rank,
                cards[3].rank,
                cards[2].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        } else if is_straight {
            Straight(straight_high.rank)
        } else if cards_by_rank.last().unwrap().len() == 3 {
            ThreeOfaKind([
                cards_by_rank[2][0].rank,
                cards_by_rank[1][0].rank,
                cards_by_rank[0][0].rank,
            ])
        } else if cards_by_rank.last().unwrap().len() == 2
            && cards_by_rank[cards_by_rank.len() - 2].len() == 2
        {
            TwoPair([
                cards_by_rank[2][0].rank,
                cards_by_rank[1][0].rank,
                cards_by_rank[0][0].rank,
            ])
        } else if cards_by_rank.last().unwrap().len() == 2 {
            OnePair([
                cards_by_rank[3][0].rank,
                cards_by_rank[2][0].rank,
                cards_by_rank[1][0].rank,
                cards_by_rank[0][0].rank,
            ])
        } else {
            HighCard([
                cards[4].rank,
                cards[3].rank,
                cards[2].rank,
                cards[1].rank,
                cards[0].rank,
            ])
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        return None;
    }
    if hands.iter().any(|h| Card::five_cards(h).is_err()) {
        return None;
    }

    let mut categoried_hands = hands
        .iter()
        .map(|&h| (Category::from(&Card::five_cards(h).unwrap()), h))
        .collect::<Vec<_>>();
    categoried_hands.sort_by_key(|x| x.0);
    let results =
        categoried_hands
            .into_iter()
            .fold(Vec::<(Category, &'a str)>::new(), |mut acc, val| {
                if acc.is_empty() || acc[0].0 < val.0 {
                    let v = vec![val];
                    v
                } else {
                    acc.push(val);
                    acc
                }
            });
    Some(results.into_iter().map(|x| x.1).collect())
}
