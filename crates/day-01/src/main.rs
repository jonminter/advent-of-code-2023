use std::io;

const BASE_10: u32 = 10;
const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const MAX_DIGIT_WORD_LEN: usize = 5;

enum DigitsRead {
    NoDigits,
    OneDigit(u32),
    AtLeastTwoDigits(u32, u32),
}
impl DigitsRead {
    fn push_digit(self, digit: u32) -> Self {
        match self {
            DigitsRead::NoDigits => DigitsRead::OneDigit(digit),
            DigitsRead::OneDigit(first_digit) => DigitsRead::AtLeastTwoDigits(first_digit, digit),
            DigitsRead::AtLeastTwoDigits(first_digit, _) => {
                DigitsRead::AtLeastTwoDigits(first_digit, digit)
            }
        }
    }

    fn try_calculate_two_digit_number(self) -> Result<u32, String> {
        match self {
            DigitsRead::NoDigits => Err("No digits read!".to_string()),
            DigitsRead::OneDigit(first_digit) => Ok(first_digit * BASE_10 + first_digit),
            DigitsRead::AtLeastTwoDigits(first_digit, last_digit) => {
                Ok(first_digit * BASE_10 + last_digit)
            }
        }
    }
}

fn get_two_digit_number_from_line(line_str: &str) -> Result<u32, String> {
    let mut digits_read = DigitsRead::NoDigits;
    for (i, c) in line_str.chars().enumerate() {
        if c.is_digit(BASE_10) {
            digits_read = digits_read.push_digit(c.to_digit(BASE_10).unwrap());
            continue;
        }

        let slice_start = if i > MAX_DIGIT_WORD_LEN {
            i - MAX_DIGIT_WORD_LEN
        } else {
            0
        };
        let slice_end = i + 1;

        for (digit_minus_one, digit_word) in DIGIT_WORDS.iter().enumerate() {
            if line_str[slice_start..slice_end].ends_with(digit_word) {
                digits_read = digits_read.push_digit(digit_minus_one as u32 + 1);
                break;
            }
        }
    }
    digits_read.try_calculate_two_digit_number()
}

fn main() {
    let lines = io::stdin().lines();

    let mut sum = 0;

    for (line_num, line_res) in lines.enumerate() {
        let line = line_res.unwrap_or_else(|_| panic!("LINE {}: Failed to read line!", line_num));
        let two_digit_num = get_two_digit_number_from_line(&line)
            .unwrap_or_else(|_| panic!("LINE {}: Failed to parse line!", line_num));
        sum += two_digit_num;
    }

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod test {
    const TEST_CASES: [(&str, u32); 12] = [
        // PART 1
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
        ("two1nine", 29),
        // PART 2
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
        // EDGE CASES
        ("twone", 21),
    ];

    #[test]
    fn test_get_two_digit_number_from_line() {
        for (line, expected) in TEST_CASES.iter() {
            let actual = super::get_two_digit_number_from_line(line).unwrap();
            assert_eq!(*expected, actual);
        }
    }
}
