use std::str::FromStr;

const CARDS: &str = include_str!("../../data/p0054_poker.txt");
const NUM_CARDS: usize = 5;

pub fn solve() -> i64 {
    let mut count = 0;

    for game in CARDS.split('\n') {
        let mut cards = game.split_whitespace();

        let hand1 = parse_hand(&mut cards);
        let hand2 = parse_hand(&mut cards);

        let s1 = score(&hand1);
        let s2 = score(&hand2);

        if s1 > s2 {
            count += 1;
        }
    }

    count
}

#[derive(Clone, Copy)]
struct Card {
    rank: u8,
    suit: Suit,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard([u8; NUM_CARDS]),
    OnePair {
        pair: u8,
        kickers: [u8; NUM_CARDS - 2],
    },
    TwoPair {
        high: u8,
        low: u8,
        kicker: u8,
    },
    ThreeOfAKind(u8),
    Straight(u8),
    Flush([u8; NUM_CARDS]),
    FullHouse(u8),
    FourOfAKind(u8),
    StraightFlush(u8),
    RoyalFlush,
}

type Cards = [Card; NUM_CARDS];

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Suit::*;

        let bytes = s.as_bytes();

        if bytes.len() != 2 {
            return Err(format!("the length of \"{s}\" is not 2"));
        }

        let rank = match bytes[0] {
            b'A' => 1,
            digit @ b'2'..=b'9' => digit - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            _ => return Err(format!("\"{}\" is not one of '0', '1', ..., '9', 'T', 'J', 'Q', 'K'", bytes[0])),
        };

        let suit = match bytes[1] {
            b'S' => Spade,
            b'C' => Club,
            b'D' => Diamond,
            b'H' => Heart,
            _ => return Err(format!("\"{}\" is not one of 'S', 'C', 'D', 'H'", bytes[1])),
        };

        Ok(Card { rank, suit })
    }
}

fn parse_hand<'a>(cards: &mut impl Iterator<Item = &'a str>) -> Cards {
    let mut hand: Cards = std::array::from_fn(|_| cards.next().unwrap().parse().unwrap());

    hand.sort_by_key(|&card| card.rank);

    hand
}

fn has_flush(cards: &Cards) -> bool {
    let suit = cards[0].suit;

    cards.iter().all(|&card| card.suit == suit)
}

fn has_straight(sorted_cards: &Cards) -> bool {
    if (1..NUM_CARDS - 1).any(|i| sorted_cards[i].rank + 1 != sorted_cards[i + 1].rank) {
        return false
    }

    sorted_cards[0].rank + 1 == sorted_cards[1].rank                // From [1, 2, 3, 4, 5] to [9, 10, 11, 12, 13]
        || sorted_cards[0].rank == 1 && sorted_cards[1].rank == 10  // [1, 10, 11, 12, 13]
}

fn has_straight_flush(sorted_cards: &Cards) -> bool {
    has_flush(sorted_cards) && has_straight(sorted_cards)
}

fn is_royal_flush(sorted_cards: &Cards) -> bool {
    sorted_cards[0].rank == 1
        && sorted_cards[1].rank == 10
        && has_straight_flush(sorted_cards)
}

fn four_of_a_kind(sorted_cards: &Cards) -> Option<u8> {
    let aaaab = sorted_cards[0].rank == sorted_cards[3].rank;
    let abbbb = sorted_cards[1].rank == sorted_cards[4].rank;

    if aaaab || abbbb {
        Some(ace_high(sorted_cards[1].rank))
    } else {
        None
    }
}

fn full_house(sorted_cards: &Cards) -> Option<u8> {
    let aaabb
        = sorted_cards[0].rank == sorted_cards[2].rank
        && sorted_cards[2].rank != sorted_cards[3].rank
        && sorted_cards[3].rank == sorted_cards[4].rank;
    
    let aabbb
        = sorted_cards[0].rank == sorted_cards[1].rank
        && sorted_cards[1].rank != sorted_cards[2].rank
        && sorted_cards[2].rank == sorted_cards[4].rank;
    
    if aaabb || aabbb {
        Some(ace_high(sorted_cards[2].rank))
    } else {
        None
    }
}

fn three_of_a_kind(sorted_cards: &Cards) -> Option<u8> {
    let aaabc
        = sorted_cards[0].rank == sorted_cards[2].rank
        && sorted_cards[2].rank != sorted_cards[3].rank
        && sorted_cards[3].rank != sorted_cards[4].rank;
    
    let abbbc
        = sorted_cards[0].rank != sorted_cards[1].rank
        && sorted_cards[1].rank == sorted_cards[3].rank
        && sorted_cards[3].rank != sorted_cards[4].rank;

    let abccc
        = sorted_cards[0].rank != sorted_cards[1].rank
        && sorted_cards[1].rank != sorted_cards[2].rank
        && sorted_cards[2].rank == sorted_cards[4].rank;
    
    if aaabc || abbbc || abccc {
        Some(ace_high(sorted_cards[2].rank))
    } else {
        None
    }
}

fn two_pair(sorted_cards: &Cards) -> Option<(u8, u8, u8)> {
    let is_pair: [bool; NUM_CARDS - 1]
        = std::array::from_fn(|i| sorted_cards[i].rank == sorted_cards[i + 1].rank);
    
    let aabbc
        = is_pair[0]
        && !is_pair[1]
        && is_pair[2]
        && !is_pair[3];

    let aabcc
        = is_pair[0]
        && !is_pair[1]
        && !is_pair[2]
        && is_pair[3];
    
    let abbcc
        = !is_pair[0]
        && is_pair[1]
        && !is_pair[2]
        && is_pair[3];
    
    let kicker_idx
        = if aabbc {
            4
        } else if aabcc {
            2
        } else if abbcc {
            0
        } else {
            return None
        };

    let kicker = ace_high(sorted_cards[kicker_idx].rank);
    
    let pair1 = ace_high(sorted_cards[1].rank);
    let pair2 = ace_high(sorted_cards[3].rank);

    let high = pair1.max(pair2);
    let low = pair1.min(pair2);

    Some((high, low, kicker))
}

fn one_pair(sorted_cards: &Cards) -> Option<(u8, [u8; NUM_CARDS - 2])> {    
    let pair_pos = (0..NUM_CARDS - 1).find(|&i| sorted_cards[i].rank == sorted_cards[i + 1].rank)?;

    if (0..NUM_CARDS - 1).any(|i| i != pair_pos && sorted_cards[i].rank == sorted_cards[i + 1].rank) {
        return None;
    }

    let pair = ace_high(sorted_cards[pair_pos].rank);
    
    let mut kickers = [0; NUM_CARDS - 2];
    let mut j = 0;

    for i in 0..NUM_CARDS {
        if i == pair_pos || i == pair_pos + 1 {
            continue;
        }

        kickers[j] = ace_high(sorted_cards[i].rank);
        j += 1;
    }

    kickers.sort_unstable_by(|a, b| b.cmp(a));

    Some((pair, kickers))
}

fn score(sorted_cards: &Cards) -> Hand {
    use Hand::*;
    
    // RoyalFlush
    if is_royal_flush(sorted_cards) {
        return RoyalFlush;
    }

    // StraightFlush
    if has_straight_flush(sorted_cards) {
        return StraightFlush(sorted_cards[NUM_CARDS - 1].rank);
    }

    // FourOfAKind
    if let Some(quad) = four_of_a_kind(sorted_cards) {
        return FourOfAKind(quad);
    }

    // FullHouse
    if let Some(triple) = full_house(sorted_cards) {
        return FullHouse(triple);
    }

    // Flush
    if has_flush(sorted_cards) {
        return Flush(descending_ranks(sorted_cards));
    }

    // Straight
    if has_straight(sorted_cards) {
        return Straight(straight_high(sorted_cards));
    }

    // ThreeOfAKind
    if let Some(triple) = three_of_a_kind(sorted_cards) {
        return ThreeOfAKind(triple);
    }

    // TwoPair
    if let Some((high, low, kicker)) = two_pair(sorted_cards) {
        return TwoPair { high, low, kicker };
    }

    // OnePair
    if let Some((pair, kickers)) = one_pair(sorted_cards) {
        return OnePair { pair, kickers };
    }

    // HighCard
    HighCard(descending_ranks(sorted_cards))
}

fn ace_high(rank: u8) -> u8 {
    if rank == 1 {
        14
    } else {
        rank
    }
}

fn descending_ranks(cards: &Cards) -> [u8; NUM_CARDS] {
    let mut ranks = cards.map(|card| ace_high(card.rank));
    ranks.sort_unstable_by(|a, b| b.cmp(a));
    ranks
}

fn straight_high(straight: &Cards) -> u8 {
    if straight[0].rank == 1 && straight[1].rank == 10 {
        14
    } else {
        straight[NUM_CARDS - 1].rank
    }
}
