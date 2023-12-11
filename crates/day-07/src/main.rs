mod card_types {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Debug, Ord, PartialOrd)]
    pub(crate) struct Card {
        rank: u8,
        label: char,
    }
    impl Card {
        pub(crate) fn new(label: char) -> Result<Self, String> {
            match label {
                'A' => Ok(Self { label, rank: 14 }),
                'K' => Ok(Self { label, rank: 13 }),
                'Q' => Ok(Self { label, rank: 12 }),
                'J' => Ok(Self { label, rank: 11 }),
                'T' => Ok(Self { label, rank: 10 }),
                _ => {
                    let rank = label
                        .to_digit(10)
                        .ok_or_else(|| format!("Failed to parse card label '{}': char was not A,K,Q,J or T so expected a digit", label))?;
                    if !(2..=9).contains(&rank) {
                        return Err(format!("Invalid card label '{}'", label));
                    }
                    Ok(Self {
                        label,
                        rank: rank as u8,
                    })
                }
            }
        }
    }

    fn hand_type_rank(hand_type: &HandType) -> u8 {
        match hand_type {
            HandType::FiveOfAKind => 0,
            HandType::FourOfAKind => 1,
            HandType::FullHouse => 2,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 4,
            HandType::OnePair => 5,
            HandType::HighCard => 6,
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub(crate) enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }
    impl PartialOrd for HandType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(hand_type_rank(self).cmp(&hand_type_rank(other)))
        }
    }
    impl Ord for HandType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            hand_type_rank(self).cmp(&hand_type_rank(other))
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub(crate) struct Hand {
        hand_type: HandType,
        cards: Vec<Card>,
        bid: u32,
    }
    impl Hand {
        pub(crate) fn new(cards: Vec<Card>, bid: u32) -> Result<Self, String> {
            if cards.len() != 5 {
                return Err(format!("Expected 5 cards, got {}", cards.len()));
            }

            Ok(Self {
                hand_type: Self::determine_hand_type(cards.as_slice()),
                cards,
                bid,
            })
        }

        pub(crate) fn bid(&self) -> u32 {
            self.bid
        }

        fn determine_hand_type(cards: &[Card]) -> HandType {
            let mut hand_kind_counts = HashMap::new();
            for card in cards {
                let count = hand_kind_counts.entry(card.rank).or_insert(0);
                *count += 1;
            }

            let mut hand_kind_counts = hand_kind_counts.iter().collect::<Vec<_>>();
            hand_kind_counts
                .sort_by(|(_, a_kind_count), (_, b_kind_count)| b_kind_count.cmp(a_kind_count));

            let max_count = hand_kind_counts[0].1;

            if *max_count == 5 {
                HandType::FiveOfAKind
            } else if *max_count == 4 {
                HandType::FourOfAKind
            } else if *max_count == 3 {
                assert!(
                    hand_kind_counts.len() >= 2,
                    "Expected at least 2 hand kinds since we have at least 3 of a kind"
                );
                let second_max_count = hand_kind_counts[1].1;

                if *second_max_count == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            } else if *max_count == 2 {
                assert!(
                    hand_kind_counts.len() >= 2,
                    "Expected at least 2 hand kinds since we have at least one pair"
                );
                let second_max_count = hand_kind_counts[1].1;

                if *second_max_count == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            } else {
                HandType::HighCard
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
            if self.hand_type == other.hand_type {
                other.cards.cmp(&self.cards)
            } else {
                self.hand_type.cmp(&other.hand_type)
            }
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

        Hand::new(cards, bid)
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
        let mut hands = TEST_INPUT
            .lines()
            .map(|line| crate::parse::parse_hand_line(line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let expected_hands = [
            Hand::new(
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
    fn test_orders_hand_kinds_correctly() {
        let hands = [
            Hand::new(
                vec![
                    Card::new('A').unwrap(),
                    Card::new('A').unwrap(),
                    Card::new('A').unwrap(),
                    Card::new('K').unwrap(),
                    Card::new('K').unwrap(),
                ],
                765,
            ),
            Hand::new(
                vec![
                    Card::new('A').unwrap(),
                    Card::new('A').unwrap(),
                    Card::new('A').unwrap(),
                    Card::new('2').unwrap(),
                    Card::new('J').unwrap(),
                ],
                1,
            ),
        ];
        println!("{:?}", hands);

        assert!(hands[0] < hands[1]);
    }

    #[test]
    fn test_total_winnings() {
        let mut hands = TEST_INPUT
            .lines()
            .map(|line| crate::parse::parse_hand_line(line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(super::get_total_winnings(hands), 6440);
    }
}
