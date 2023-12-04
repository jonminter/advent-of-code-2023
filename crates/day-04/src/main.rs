use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    ops::Deref,
};

struct CharAt {
    column: usize,
    c: char,
}
impl From<CharAt> for char {
    fn from(value: CharAt) -> Self {
        value.c
    }
}
impl Deref for CharAt {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    number: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: Vec<u32>,
}
impl Card {
    fn points(&self) -> u32 {
        let mut points = 0;
        for card_number in &self.card_numbers {
            if self.winning_numbers.contains(card_number) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }
        points
    }

    fn matching_number_count(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|card_number| self.winning_numbers.contains(card_number))
            .count()
    }
}

const CARD_HEADER: &str = "Card";

fn get_error_msg_with_col(col_num: usize, msg: &str) -> String {
    format!("COL {}: {}", col_num, msg)
}

fn get_parse_col(char_iter: &mut Peekable<impl Iterator<Item = CharAt>>) -> Result<usize, String> {
    char_iter
        .peek()
        .map(|c| c.column)
        .ok_or("Empty line while getting parse col".to_string())
}

fn try_consume_number(
    char_iter: &mut Peekable<impl Iterator<Item = CharAt>>,
) -> Result<u32, String> {
    let start_col = get_parse_col(char_iter)?;

    let mut number_str = String::new();
    while let Some(c) = char_iter.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        let c = char_iter.next().unwrap();
        number_str.push(c.into());
    }

    if number_str.is_empty() {
        return Err(get_error_msg_with_col(start_col, "Expected number"));
    }

    number_str
        .parse::<u32>()
        .map_err(|e| get_error_msg_with_col(start_col, &format!("Failed to parse number: {}", e)))
}

fn try_consume_whitespace(
    char_iter: &mut Peekable<impl Iterator<Item = CharAt>>,
) -> Result<usize, String> {
    let start_col = get_parse_col(char_iter)?;

    let mut whitespace_count = 0;

    while let Some(c) = char_iter.peek() {
        if !c.is_whitespace() {
            break;
        }
        char_iter.next();
        whitespace_count += 1;
    }

    if whitespace_count == 0 {
        return Err(get_error_msg_with_col(start_col, "Expected whitespace"));
    }

    Ok(whitespace_count)
}

fn try_consume_list_of_numbers(
    char_iter: &mut Peekable<impl Iterator<Item = CharAt>>,
    end_of_list_char: Option<char>,
) -> Result<Vec<u32>, String> {
    let mut numbers = Vec::new();
    loop {
        if char_iter.peek().is_none() {
            break;
        }

        let _ = try_consume_whitespace(char_iter)?;

        match (char_iter.peek(), end_of_list_char) {
            (Some(c), Some(end_of_list_char)) if c.deref() == &end_of_list_char => {
                char_iter.next();
                break;
            }
            (None, _) => break,
            _ => {}
        }

        let number = try_consume_number(char_iter)?;
        numbers.push(number);
    }
    Ok(numbers)
}

fn get_error_msg_with_line(line_num: usize, msg: &str) -> String {
    format!("LINE {}: {}", line_num, msg)
}

fn parse_card_line(line_number: usize, line: &str) -> Result<Card, String> {
    let mut char_iter = line
        .chars()
        .enumerate()
        .map(|(col, c)| CharAt { column: col, c })
        .peekable();

    if char_iter.peek().is_none() {
        return Err(get_error_msg_with_line(line_number, "Empty line"));
    }

    let first_4: String = char_iter.by_ref().take(4).map(|c| c.c).collect();
    if first_4 != CARD_HEADER {
        return Err(format!(
            "Expected \"{}\" at start of line, got \"{}\"",
            CARD_HEADER, first_4
        ));
    }

    let _ = try_consume_whitespace(&mut char_iter)
        .map_err(|e| get_error_msg_with_line(line_number, &e))?;

    let card_number =
        try_consume_number(&mut char_iter).map_err(|e| get_error_msg_with_line(line_number, &e))?;

    let next_char = char_iter.next().ok_or_else(|| {
        get_error_msg_with_line(
            line_number,
            "Expected another char after card number, got end of line",
        )
    })?;
    if *next_char != ':' {
        return Err(get_error_msg_with_line(
            line_number,
            &format!("Expected \":\" after card number, got \"{}\"", *next_char),
        ));
    }

    let winning_numbers = try_consume_list_of_numbers(&mut char_iter, Some('|'))
        .map_err(|e| get_error_msg_with_line(line_number, &e))?;
    let card_numbers = try_consume_list_of_numbers(&mut char_iter, None)
        .map_err(|e| get_error_msg_with_line(line_number, &e))?;

    assert!(char_iter.next().is_none());

    Ok(Card {
        number: card_number,
        winning_numbers: winning_numbers.into_iter().collect(),
        card_numbers,
    })
}

struct PointsWon(u32);
impl Deref for PointsWon {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CardsWon(u32);
impl Deref for CardsWon {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn calculate_points_and_total_cards(
    cards: &mut impl Iterator<Item = Result<Card, String>>,
) -> Result<(PointsWon, CardsWon), String> {
    let mut num_copies_won = HashMap::new();

    let mut total_cards = 0;
    let mut total_points = 0;
    for card in cards {
        let card = card?;

        let num_copies_of_this_card = *num_copies_won.entry(card.number).or_insert(0) + 1;
        let num_matching_numbers = card.matching_number_count();

        if num_matching_numbers > 0 {
            for i in 1..=num_matching_numbers {
                num_copies_won
                    .entry(card.number + i as u32)
                    .and_modify(|n| *n += num_copies_of_this_card)
                    .or_insert(num_copies_of_this_card);
            }
        }
        total_cards += num_copies_of_this_card;
        total_points += card.points();
    }
    Ok((
        PointsWon(total_points),
        CardsWon(total_cards.try_into().unwrap()),
    ))
}

fn main() {
    let mut cards = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let line = line.unwrap();
            parse_card_line(line_num, &line)
                .map_err(|e| format!("Error parsing line {}: {}", line_num, e))
        });

    let (points_won, cards_won) =
        calculate_points_and_total_cards(&mut cards).expect("Failed to calculate cards won");
    println!("Points won: {}", *points_won);
    println!("Cards won: {}", *cards_won);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::Card;

    fn get_test_cases() -> Vec<(&'static str, Card)> {
        vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", {
                Card {
                    number: 1,
                    winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                    card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                }
            }),
            (
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                Card {
                    number: 2,
                    winning_numbers: HashSet::from([13, 32, 20, 16, 61]),
                    card_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
            ),
            (
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                Card {
                    number: 3,
                    winning_numbers: HashSet::from([1, 21, 53, 59, 44]),
                    card_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                },
            ),
            (
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                Card {
                    number: 4,
                    winning_numbers: HashSet::from([41, 92, 73, 84, 69]),
                    card_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                },
            ),
            (
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
                Card {
                    number: 5,
                    winning_numbers: HashSet::from([87, 83, 26, 28, 32]),
                    card_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                },
            ),
            (
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                Card {
                    number: 6,
                    winning_numbers: HashSet::from([31, 18, 13, 56, 72]),
                    card_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                },
            ),
        ]
    }

    #[test]
    fn test_parses_card_lines() {
        for (line_num, (line, expected_card)) in get_test_cases().iter().enumerate() {
            let card = super::parse_card_line(line_num, line)
                .map_err(|e| format!("Error parsing line: \"{}\": {}", line, e))
                .unwrap();
            assert_eq!(
                &card, expected_card,
                "Parsed card did not match expected while parsing line: \"{}\"",
                line
            );
        }
    }

    #[test]
    fn test_card_points_sum() {
        let cards = get_test_cases()
            .into_iter()
            .map(|(_, card)| card)
            .collect::<Vec<_>>();
        let points_sum = cards.iter().map(|card| card.points()).sum::<u32>();
        assert_eq!(points_sum, 13);
    }

    #[test]
    fn test_cards_won() {
        let mut cards = get_test_cases().into_iter().map(|(_, card)| Ok(card));

        let (_, cards_won) = super::calculate_points_and_total_cards(&mut cards).unwrap();

        assert_eq!(*cards_won, 30);
    }
}
