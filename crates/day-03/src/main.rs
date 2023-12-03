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

#[derive(PartialEq, Debug, Eq)]
enum SymbolType {
    Gear,
    Other,
}

#[derive(PartialEq, Debug)]
struct SymbolNode {
    sym_type: SymbolType,
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Line {
    symbols: Vec<SymbolNode>,
    numbers: Vec<NumberNode>,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct PartNumber(u32);
#[derive(PartialEq, Eq, Debug, Hash)]
struct GearRatio(u32);

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
) -> Vec<PartNumber> {
    let search_symbols: Vec<&SymbolNode> = prev_line
        .map(|l| l.symbols.iter())
        .unwrap_or_default()
        .chain(current_line.symbols.iter())
        .chain(next_line.map(|l| l.symbols.iter()).unwrap_or_default())
        .collect();

    current_line
        .numbers
        .iter()
        .filter(|number| {
            search_symbols
                .iter()
                .filter(|symbol| number.is_adjacent_to(symbol))
                .count()
                > 0
        })
        .map(|number| PartNumber(number.value))
        .collect()
}

fn find_gear_ratios(
    prev_line: Option<&Line>,
    current_line: &Line,
    next_line: Option<&Line>,
) -> Vec<GearRatio> {
    let search_numbers: Vec<&NumberNode> = prev_line
        .map(|l| l.numbers.iter())
        .unwrap_or_default()
        .chain(current_line.numbers.iter())
        .chain(next_line.map(|l| l.numbers.iter()).unwrap_or_default())
        .collect();

    let gear_symbols = current_line
        .symbols
        .iter()
        .filter(|s| s.sym_type == SymbolType::Gear);

    let mut gear_ratios = vec![];
    for gear_sym in gear_symbols {
        let adjacent_numbers: Vec<&&NumberNode> = search_numbers
            .iter()
            .filter(|number| number.is_adjacent_to(gear_sym))
            .collect();

        if adjacent_numbers.len() == 2 {
            gear_ratios.push(GearRatio(
                adjacent_numbers[0].value * adjacent_numbers[1].value,
            ));
        }
    }

    gear_ratios
}

fn find_all_part_numbers_and_gear_ratios(
    mut lines: impl Iterator<Item = Line>,
) -> (Vec<PartNumber>, Vec<GearRatio>) {
    let maybe_first_line = lines.next();
    match maybe_first_line {
        Some(first_line) => {
            let mut prev_line = None;
            let mut current_line = first_line;

            let mut part_numbers = vec![];
            let mut gear_ratios = vec![];

            loop {
                let next_line = lines.next();
                part_numbers.extend(find_part_numbers(
                    prev_line.as_ref(),
                    &current_line,
                    next_line.as_ref(),
                ));
                gear_ratios.extend(find_gear_ratios(
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

            (part_numbers, gear_ratios)
        }
        None => (vec![], vec![]),
    }
}

const GEAR_SYMBOL: char = '*';
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
                let sym_type = if c == GEAR_SYMBOL {
                    SymbolType::Gear
                } else {
                    SymbolType::Other
                };
                symbols.push(SymbolNode { sym_type, x, y });
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

    let (part_numbers, gear_ratios) = find_all_part_numbers_and_gear_ratios(lines);

    let part_numbers_sum: u32 = part_numbers.iter().map(|p| p.0).sum();
    let gear_ratios_sum: u32 = gear_ratios.iter().map(|g| g.0).sum();

    println!("Sum of part numbers: {}", part_numbers_sum);
    println!("Sum of gear ratios: {}", gear_ratios_sum);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        find_all_part_numbers_and_gear_ratios, parse_line_into_symbols_and_numbers, GearRatio,
        Line, NumberNode, PartNumber, SymbolNode, SymbolType,
    };

    fn get_expected_part_numbers() -> Vec<PartNumber> {
        [467, 35, 633, 617, 592, 755, 664, 598]
            .iter()
            .map(|n| PartNumber(*n))
            .collect()
    }

    fn get_expected_gear_ratios() -> Vec<GearRatio> {
        vec![GearRatio(16345), GearRatio(451490)]
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
                    symbols: vec![SymbolNode {
                        sym_type: crate::SymbolType::Gear,
                        x: 3,
                        y: 0,
                    }],
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
                    symbols: vec![SymbolNode {
                        sym_type: SymbolType::Other,
                        x: 6,
                        y: 0,
                    }],
                    numbers: vec![],
                },
            ),
            (
                "617*......",
                Line {
                    symbols: vec![SymbolNode {
                        sym_type: SymbolType::Gear,
                        x: 3,
                        y: 0,
                    }],
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
                    symbols: vec![SymbolNode {
                        sym_type: SymbolType::Other,
                        x: 5,
                        y: 0,
                    }],
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
                    symbols: vec![
                        SymbolNode {
                            sym_type: SymbolType::Other,
                            x: 3,
                            y: 0,
                        },
                        SymbolNode {
                            sym_type: SymbolType::Gear,
                            x: 5,
                            y: 0,
                        },
                    ],
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
                    symbols: vec![SymbolNode {
                        sym_type: SymbolType::Gear,
                        x: 3,
                        y: 0,
                    }],
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
        let (part_numbers, _) = find_all_part_numbers_and_gear_ratios(
            get_test_cases().into_iter().map(|(_, line)| line),
        );

        let expected_part_numbers = get_expected_part_numbers();

        assert_eq!(part_numbers.len(), expected_part_numbers.len());
        assert_eq!(
            part_numbers.into_iter().collect::<HashSet<_>>(),
            expected_part_numbers.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_sums_part_numbers() {
        let (part_numbers, _) = find_all_part_numbers_and_gear_ratios(
            get_test_cases().into_iter().map(|(_, line)| line),
        );

        let part_numbers_sum: u32 = part_numbers.iter().map(|p| p.0).sum();

        assert_eq!(part_numbers_sum, 4361);
    }

    #[test]
    fn test_parses_gear_ratios() {
        let (_, gear_ratios) = find_all_part_numbers_and_gear_ratios(
            get_test_cases().into_iter().map(|(_, line)| line),
        );

        assert_eq!(gear_ratios, get_expected_gear_ratios());
    }

    #[test]
    fn test_sums_gear_ratios() {
        let (_, gear_ratios) = find_all_part_numbers_and_gear_ratios(
            get_test_cases().into_iter().map(|(_, line)| line),
        );

        let gear_ratios_sum: u32 = gear_ratios.iter().map(|g| g.0).sum();

        assert_eq!(gear_ratios_sum, 467835);
    }
}
