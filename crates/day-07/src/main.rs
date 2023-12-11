mod card_types {
    use std::collections::{HashMap, HashSet};

    #[derive(PartialEq, Eq, Debug, Hash, Clone)]
    pub(crate) enum Card {
        Ace,
        King,
        Queen,
        Joker,
        Tee,
        Number(u8),
    }
    impl Ord for Card {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.rank().cmp(&other.rank())
        }
    }
    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.rank().cmp(&other.rank()))
        }
    }

    impl Card {
        pub(crate) fn new(label: char) -> Result<Self, String> {
            match label {
                'A' => Ok(Card::Ace),
                'K' => Ok(Card::King),
                'Q' => Ok(Card::Queen),
                'J' => Ok(Card::Joker),
                'T' => Ok(Card::Tee),
                _ => {
                    let card_number = label
                        .to_digit(10)
                        .ok_or_else(|| format!("Failed to parse card label '{}': char was not A,K,Q,J or T so expected a digit", label))?;
                    if !(2..=9).contains(&card_number) {
                        return Err(format!("Invalid card label '{}'", label));
                    }
                    Ok(Card::Number(card_number as u8))
                }
            }
        }

        pub(crate) fn rank(&self) -> u8 {
            match self {
                Self::Ace => 14,
                Self::King => 13,
                Self::Queen => 12,
                Self::Joker => 1,
                Self::Tee => 10,
                Self::Number(rank) => *rank,
            }
        }
    }

    fn hand_type_rank(hand_type: &Hand) -> u8 {
        match hand_type {
            Hand::FiveOfAKind(_, _) => 0,
            Hand::FourOfAKind(_, _) => 1,
            Hand::FullHouse(_, _) => 2,
            Hand::ThreeOfAKind(_, _) => 3,
            Hand::TwoPair(_, _) => 4,
            Hand::OnePair(_, _) => 5,
            Hand::HighCard(_, _) => 6,
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub(crate) enum Hand {
        FiveOfAKind(Vec<Card>, u32),
        FourOfAKind(Vec<Card>, u32),
        FullHouse(Vec<Card>, u32),
        ThreeOfAKind(Vec<Card>, u32),
        TwoPair(Vec<Card>, u32),
        OnePair(Vec<Card>, u32),
        HighCard(Vec<Card>, u32),
    }
    impl Hand {
        pub(crate) fn bid(&self) -> u32 {
            match self {
                Hand::FiveOfAKind(_, bid) => *bid,
                Hand::FourOfAKind(_, bid) => *bid,
                Hand::FullHouse(_, bid) => *bid,
                Hand::ThreeOfAKind(_, bid) => *bid,
                Hand::TwoPair(_, bid) => *bid,
                Hand::OnePair(_, bid) => *bid,
                Hand::HighCard(_, bid) => *bid,
            }
        }

        fn cards(&self) -> &Vec<Card> {
            match self {
                Hand::FiveOfAKind(cards, _) => cards,
                Hand::FourOfAKind(cards, _) => cards,
                Hand::FullHouse(cards, _) => cards,
                Hand::ThreeOfAKind(cards, _) => cards,
                Hand::TwoPair(cards, _) => cards,
                Hand::OnePair(cards, _) => cards,
                Hand::HighCard(cards, _) => cards,
            }
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match hand_type_rank(self).cmp(&hand_type_rank(other)) {
                std::cmp::Ordering::Equal => other.cards().cmp(self.cards()),
                ordering => ordering,
            }
        }
    }

    impl Hand {
        fn hand_from_cards(
            actual_cards: Vec<Card>,
            maybe_jokers_replaced: Option<&Vec<Card>>,
            bid: u32,
        ) -> Hand {
            let cards_to_determine_type = maybe_jokers_replaced.unwrap_or(&actual_cards);
            assert!(cards_to_determine_type.len() == 5);

            let mut hand_kind_counts = HashMap::new();
            for card in cards_to_determine_type {
                let count = hand_kind_counts.entry(card).or_insert(0);
                *count += 1;
            }

            let mut hand_kind_counts = hand_kind_counts.iter().collect::<Vec<_>>();
            hand_kind_counts
                .sort_by(|(_, a_kind_count), (_, b_kind_count)| a_kind_count.cmp(b_kind_count));

            let max_count: u32 = *hand_kind_counts.pop().map(|(_, count)| count).unwrap();
            let second_most_count: Option<u32> = hand_kind_counts.pop().map(|(_, count)| *count);

            match (max_count, second_most_count) {
                (5, _) => Hand::FiveOfAKind(actual_cards, bid),
                (4, _) => Hand::FourOfAKind(actual_cards, bid),
                (3, Some(2)) => Hand::FullHouse(actual_cards, bid),
                (3, _) => Hand::ThreeOfAKind(actual_cards, bid),
                (2, Some(2)) => Hand::TwoPair(actual_cards, bid),
                (2, _) => Hand::OnePair(actual_cards, bid),
                _ => Hand::HighCard(actual_cards, bid),
            }
        }

        fn find_max_hand_with_joker_sub(cards: &[Card], bid: u32) -> Hand {
            let num_jokers = cards.iter().filter(|c| **c == Card::Joker).count();

            let orig_cards = cards.to_vec();
            let orig_hand = Hand::hand_from_cards(orig_cards.clone(), None, bid);
            let mut hand = orig_hand.clone();

            if num_jokers == 0 {
                return hand;
            }
            let other_card_types = cards
                .iter()
                .filter(|c| **c != Card::Joker)
                .clone()
                .collect::<HashSet<_>>();

            for card_to_replace_with_joker in other_card_types {
                let new_cards = cards
                    .iter()
                    .map(|c| {
                        if *c == Card::Joker {
                            card_to_replace_with_joker
                        } else {
                            c
                        }
                    })
                    .cloned()
                    .collect();
                let new_hand = Hand::hand_from_cards(orig_cards.clone(), Some(&new_cards), bid);

                if new_hand < hand {
                    hand = new_hand;
                }
            }

            hand
        }

        pub(crate) fn new(cards: &[Card], bid: u32) -> Result<Self, String> {
            if cards.len() != 5 {
                return Err(format!("Expected 5 cards, got {}", cards.len()));
            }

            Ok(Self::find_max_hand_with_joker_sub(cards, bid))
        }
    }
}

mod parse {
    use crate::card_types::{Card, Hand};

    pub(crate) fn parse_hand_line(line: &str) -> Result<Hand, String> {
        let mut cards = Vec::new();
        let (cards_str, bid_str) = line
            .split_once(' ')
            .ok_or("Expected cards & bid separated by a space")?;

        for c in cards_str.chars() {
            cards.push(Card::new(c)?);
        }

        let bid = bid_str
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse bid '{}': {}", bid_str, e))?;

        Hand::new(&cards, bid)
    }
}

fn get_total_winnings(mut hands: Vec<card_types::Hand>) -> u32 {
    hands.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.bid() * (i as u32 + 1);
    }
    total_winnings
}

fn main() {
    let hands = std::io::stdin()
        .lines()
        .map(|line| parse::parse_hand_line(&line.unwrap()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("Total winnings: {}", get_total_winnings(hands));
}

#[cfg(test)]
mod test {
    use crate::card_types::{Card, Hand};

    const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_parse() {
        let hands = TEST_INPUT
            .lines()
            .map(|line| crate::parse::parse_hand_line(line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let expected_hands = [
            Hand::new(
                &vec![
                    Card::new('3').unwrap(),
                    Card::new('2').unwrap(),
                    Card::new('T').unwrap(),
                    Card::new('3').unwrap(),
                    Card::new('K').unwrap(),
                ],
                765,
            )
            .unwrap(),
            Hand::new(
                &vec![
                    Card::new('T').unwrap(),
                    Card::new('5').unwrap(),
                    Card::new('5').unwrap(),
                    Card::new('J').unwrap(),
                    Card::new('5').unwrap(),
                ],
                684,
            )
            .unwrap(),
            Hand::new(
                &vec![
                    Card::new('K').unwrap(),
                    Card::new('K').unwrap(),
                    Card::new('6').unwrap(),
                    Card::new('7').unwrap(),
                    Card::new('7').unwrap(),
                ],
                28,
            )
            .unwrap(),
            Hand::new(
                &vec![
                    Card::new('K').unwrap(),
                    Card::new('T').unwrap(),
                    Card::new('J').unwrap(),
                    Card::new('J').unwrap(),
                    Card::new('T').unwrap(),
                ],
                220,
            )
            .unwrap(),
            Hand::new(
                &vec![
                    Card::new('Q').unwrap(),
                    Card::new('Q').unwrap(),
                    Card::new('Q').unwrap(),
                    Card::new('J').unwrap(),
                    Card::new('A').unwrap(),
                ],
                483,
            )
            .unwrap(),
        ];

        assert_eq!(hands, expected_hands);
    }

    #[test]
    fn test_total_winnings() {
        let hands = TEST_INPUT
            .lines()
            .map(|line| crate::parse::parse_hand_line(line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(super::get_total_winnings(hands), 5905);
    }
}
