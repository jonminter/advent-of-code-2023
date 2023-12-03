#[derive(PartialEq, Debug)]
struct NumberNode {
    value: u32,
    start_x: usize,
    end_x: usize,
    y: usize,
}
impl NumberNode {
    fn adjacency_bounds(&self) -> (usize, usize, usize, usize) {
        (
            self.start_x.saturating_sub(1),
            self.end_x + 1,
            self.y.saturating_sub(1),
            self.y + 1,
        )
    }
    fn is_adjacent_to(&self, symbol: &SymbolNode) -> bool {
        let (min_x, max_x, min_y, max_y) = self.adjacency_bounds();
        symbol.x >= min_x && symbol.x <= max_x && symbol.y >= min_y && symbol.y <= max_y
    }
}
#[derive(PartialEq, Debug)]
struct SymbolNode {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Line {
    symbols: Vec<SymbolNode>,
    numbers: Vec<NumberNode>,
}

/// Find all part numbers in a line.
/// We only need the symbols that are on the current, prev & next lines (if they exist)
/// to determine if any numbers in this line are part numbers instead of comparing
/// all numbers to all symbols across the entire input.
/// We take the list of numbers and check if any are adjacent to any of the symbols
/// either up, down, left, or right or diagonally
fn find_part_numbers(
    prev_line: Option<&Line>,
    current_line: &Line,
    next_line: Option<&Line>,
) -> Vec<u32> {
    let all_symbols: Vec<&SymbolNode> = prev_line
        .map(|l| l.symbols.iter())
        .unwrap_or_default()
        .chain(current_line.symbols.iter())
        .chain(next_line.map(|l| l.symbols.iter()).unwrap_or_default())
        .collect();

    current_line
        .numbers
        .iter()
        .filter(|number| {
            all_symbols
                .iter()
                .filter(|symbol| number.is_adjacent_to(symbol))
                .count()
                > 0
        })
        .map(|number| number.value)
        .collect()
}

fn find_all_part_numbers(mut lines: impl Iterator<Item = Line>) -> Vec<u32> {
    let maybe_first_line = lines.next();
    match maybe_first_line {
        Some(first_line) => {
            let mut prev_line = None;
            let mut current_line = first_line;

            let mut part_numbers = Vec::new();

            loop {
                let next_line = lines.next();
                part_numbers.extend(find_part_numbers(
                    prev_line.as_ref(),
                    &current_line,
                    next_line.as_ref(),
                ));
                prev_line = Some(current_line);

                match next_line {
                    Some(line) => current_line = line,
                    None => break,
                }
            }

            part_numbers
        }
        None => vec![],
    }
}

const PERIOD: char = '.';
const BASE_10: u32 = 10;

struct NumberBuilder {
    y: usize,
    start_x: usize,
    end_x: usize,
    digits_buf: Vec<u32>,
}
impl NumberBuilder {
    fn new(y: usize, start_x: usize, first_digit: u32) -> Self {
        assert!(first_digit < BASE_10);
        Self {
            y,
            start_x,
            end_x: start_x,
            digits_buf: vec![first_digit],
        }
    }

    fn push_digit(&mut self, digit: u32) {
        assert!(digit < BASE_10);
        self.digits_buf.push(digit);
        self.end_x += 1;
    }

    fn build(self) -> NumberNode {
        NumberNode {
            value: self
                .digits_buf
                .iter()
                .fold(0, |acc, digit| acc * BASE_10 + digit),
            start_x: self.start_x,
            end_x: self.end_x,
            y: self.y,
        }
    }
}

fn parse_line_into_symbols_and_numbers(line: &str, y: usize) -> Line {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    let mut current_number_buf: Option<NumberBuilder> = None;
    for (x, c) in line.chars().enumerate() {
        current_number_buf = if c.is_digit(BASE_10) {
            let digit = c.to_digit(BASE_10).unwrap();
            match current_number_buf {
                Some(mut number_buf) => {
                    number_buf.push_digit(digit);
                    Some(number_buf)
                }
                None => Some(NumberBuilder::new(y, x, digit)),
            }
        } else {
            if let Some(number) = current_number_buf.map(|number_buf| number_buf.build()) {
                numbers.push(number);
            }

            if c != PERIOD {
                symbols.push(SymbolNode { x, y });
            }
            None
        }
    }
    if let Some(number) = current_number_buf.map(|number_buf| number_buf.build()) {
        numbers.push(number);
    }

    Line { symbols, numbers }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line)| parse_line_into_symbols_and_numbers(&line.unwrap(), y));

    let part_numbers = find_all_part_numbers(lines);

    let part_numbers_sum: u32 = part_numbers.iter().sum();

    println!("{}", part_numbers_sum);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        find_all_part_numbers, parse_line_into_symbols_and_numbers, Line, NumberNode, SymbolNode,
    };

    fn get_expected_part_numbers() -> Vec<u32> {
        [467, 35, 633, 617, 592, 755, 664, 598]
            .iter()
            .cloned()
            .collect()
    }

    fn get_test_cases() -> Vec<(&'static str, Line)> {
        vec![
            (
                "467..114..",
                Line {
                    symbols: vec![],
                    numbers: vec![
                        NumberNode {
                            value: 467,
                            start_x: 0,
                            end_x: 2,
                            y: 0,
                        },
                        NumberNode {
                            value: 114,
                            start_x: 5,
                            end_x: 7,
                            y: 0,
                        },
                    ],
                },
            ),
            (
                "...*......",
                Line {
                    symbols: vec![SymbolNode { x: 3, y: 0 }],
                    numbers: vec![],
                },
            ),
            (
                "..35..633.",
                Line {
                    symbols: vec![],
                    numbers: vec![
                        NumberNode {
                            value: 35,
                            start_x: 2,
                            end_x: 3,
                            y: 0,
                        },
                        NumberNode {
                            value: 633,
                            start_x: 6,
                            end_x: 8,
                            y: 0,
                        },
                    ],
                },
            ),
            (
                "......#...",
                Line {
                    symbols: vec![SymbolNode { x: 6, y: 0 }],
                    numbers: vec![],
                },
            ),
            (
                "617*......",
                Line {
                    symbols: vec![SymbolNode { x: 3, y: 0 }],
                    numbers: vec![NumberNode {
                        value: 617,
                        start_x: 0,
                        end_x: 2,
                        y: 0,
                    }],
                },
            ),
            (
                ".....+.58.",
                Line {
                    symbols: vec![SymbolNode { x: 5, y: 0 }],
                    numbers: vec![NumberNode {
                        value: 58,
                        start_x: 7,
                        end_x: 8,
                        y: 0,
                    }],
                },
            ),
            (
                "..592.....",
                Line {
                    symbols: vec![],
                    numbers: vec![NumberNode {
                        value: 592,
                        start_x: 2,
                        end_x: 4,
                        y: 0,
                    }],
                },
            ),
            (
                "......755.",
                Line {
                    symbols: vec![],
                    numbers: vec![NumberNode {
                        value: 755,
                        start_x: 6,
                        end_x: 8,
                        y: 0,
                    }],
                },
            ),
            (
                "...$.*....",
                Line {
                    symbols: vec![SymbolNode { x: 3, y: 0 }, SymbolNode { x: 5, y: 0 }],
                    numbers: vec![],
                },
            ),
            (
                ".664.598..",
                Line {
                    symbols: vec![],
                    numbers: vec![
                        NumberNode {
                            value: 664,
                            start_x: 1,
                            end_x: 3,
                            y: 0,
                        },
                        NumberNode {
                            value: 598,
                            start_x: 5,
                            end_x: 7,
                            y: 0,
                        },
                    ],
                },
            ),
        ]
    }

    fn get_parser_edge_cases() -> Vec<(&'static str, Line)> {
        vec![
            (
                ".456",
                Line {
                    symbols: vec![],
                    numbers: vec![NumberNode {
                        value: 456,
                        start_x: 1,
                        end_x: 3,
                        y: 0,
                    }],
                },
            ),
            (
                "123*456",
                Line {
                    symbols: vec![SymbolNode { x: 3, y: 0 }],
                    numbers: vec![
                        NumberNode {
                            value: 123,
                            start_x: 0,
                            end_x: 2,
                            y: 0,
                        },
                        NumberNode {
                            value: 456,
                            start_x: 4,
                            end_x: 6,
                            y: 0,
                        },
                    ],
                },
            ),
        ]
    }

    #[test]
    fn test_parses_lines() {
        let test_cases = get_test_cases()
            .into_iter()
            .chain(get_parser_edge_cases().into_iter());

        for (line_str, expected_line) in test_cases {
            let actual_line = parse_line_into_symbols_and_numbers(line_str, 0);
            assert_eq!(
                actual_line, expected_line,
                "Failed parsing LINE '{}'!",
                line_str
            );
        }
    }

    #[test]
    fn test_finds_right_part_numbers() {
        let part_numbers =
            find_all_part_numbers(get_test_cases().into_iter().map(|(_, line)| line));

        let expected_part_numbers = get_expected_part_numbers();

        assert_eq!(part_numbers.len(), expected_part_numbers.len());
        assert_eq!(
            part_numbers.into_iter().collect::<HashSet<_>>(),
            expected_part_numbers.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_sums_part_numbers() {
        let part_numbers =
            find_all_part_numbers(get_test_cases().into_iter().map(|(_, line)| line));

        let part_numbers_sum: u32 = part_numbers.iter().sum();

        assert_eq!(part_numbers_sum, 4361);
    }
}
