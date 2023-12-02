use std::io;

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
            DigitsRead::OneDigit(first_digit) => Ok(first_digit * 10 + first_digit),
            DigitsRead::AtLeastTwoDigits(first_digit, last_digit) => {
                Ok(first_digit * 10 + last_digit)
            }
        }
    }
}

const BASE_10: u32 = 10;

fn get_two_digit_number_from_line(line_str: &str) -> Result<u32, String> {
    line_str
        .chars()
        .fold(DigitsRead::NoDigits, |digits_read, c| {
            if c.is_digit(BASE_10) {
                digits_read.push_digit(c.to_digit(BASE_10).unwrap())
            } else {
                digits_read
            }
        })
        .try_calculate_two_digit_number()
}

fn main() {
    let lines = io::stdin().lines();

    let mut sum = 0;

    for (line_num, line_res) in lines.enumerate() {
        let line = line_res.expect(format!("LINE {}: Failed to read line!", line_num).as_str());
        let two_digit_num = get_two_digit_number_from_line(&line)
            .expect(format!("LINE {}: Failed to parse line!", line_num).as_str());
        sum += two_digit_num;
    }

    println!("Sum: {}", sum);
}
