use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct BallBag {
    red_balls: u32,
    green_balls: u32,
    blue_balls: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum BallColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct GameTurn {
    balls_drawn: HashMap<BallColor, u32>,
}
impl GameTurn {
    fn new() -> Self {
        GameTurn {
            balls_drawn: HashMap::new(),
        }
    }

    fn with_red(mut self, red_balls: u32) -> Result<Self, String> {
        match self.balls_drawn.get(&BallColor::Red) {
            None => {
                self.balls_drawn.insert(BallColor::Red, red_balls);
                Ok(self)
            }
            Some(_) => Err("Already set number of red balls drawn this turn".to_string()),
        }
    }

    fn with_green(mut self, green_balls: u32) -> Result<Self, String> {
        match self.balls_drawn.get(&BallColor::Green) {
            None => {
                self.balls_drawn.insert(BallColor::Green, green_balls);
                Ok(self)
            }
            Some(_) => Err("Already set number of green balls drawn this turn".to_string()),
        }
    }

    fn with_blue(mut self, blue_balls: u32) -> Result<Self, String> {
        match self.balls_drawn.get(&BallColor::Blue) {
            None => {
                self.balls_drawn.insert(BallColor::Blue, blue_balls);
                Ok(self)
            }
            Some(_) => Err("Already set number of blue balls drawn this turn".to_string()),
        }
    }

    fn num_red_drawn(&self) -> u32 {
        *self.balls_drawn.get(&BallColor::Red).unwrap_or(&0)
    }

    fn num_green_drawn(&self) -> u32 {
        *self.balls_drawn.get(&BallColor::Green).unwrap_or(&0)
    }

    fn num_blue_drawn(&self) -> u32 {
        *self.balls_drawn.get(&BallColor::Blue).unwrap_or(&0)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    number: usize,
    turns: Vec<GameTurn>,
}
impl Game {
    fn is_possible_with_bag(&self, ball_bag: &BallBag) -> bool {
        let mut max_red_drawn = 0;
        let mut max_green_drawn = 0;
        let mut max_blue_drawn = 0;
        for turn in &self.turns {
            max_red_drawn = max_red_drawn.max(turn.num_red_drawn());
            max_green_drawn = max_green_drawn.max(turn.num_green_drawn());
            max_blue_drawn = max_blue_drawn.max(turn.num_blue_drawn());
        }
        ball_bag.red_balls >= max_red_drawn
            && ball_bag.green_balls >= max_green_drawn
            && ball_bag.blue_balls >= max_blue_drawn
    }
}

fn turn_parse_error_msg(game_num: usize, turn_num: usize, msg: &str) -> String {
    format!("GAME {}, TURN {}: {}", game_num, turn_num + 1, msg)
}

fn parse_num_balls_and_color(
    game_number: usize,
    turn_number: usize,
    balls_drawn: &str,
) -> Result<(u32, &str), String> {
    let (num_balls, color) = balls_drawn.split_once(' ').ok_or_else(|| {
        turn_parse_error_msg(
            game_number,
            turn_number,
            "Expected a space separating number of balls and color",
        )
    })?;

    let num_balls = num_balls.parse::<u32>().map_err(|_| {
        turn_parse_error_msg(
            game_number,
            turn_number,
            &format!(
                "Failed to parse number of balls in turn for color {}",
                color
            ),
        )
    })?;

    Ok((num_balls, color))
}

fn parse_turn_from_str(
    game_number: usize,
    turn_number: usize,
    turn_to_parse: &str,
) -> Result<GameTurn, String> {
    let balls_drawn_to_parse = turn_to_parse.split(", ");
    let mut turn = GameTurn::new();

    for balls_drawn in balls_drawn_to_parse {
        let (num_balls, color) = parse_num_balls_and_color(game_number, turn_number, balls_drawn)?;

        if color == "red" {
            turn = turn
                .with_red(num_balls)
                .map_err(|msg| turn_parse_error_msg(game_number, turn_number, &msg))?;
        } else if color == "green" {
            turn = turn
                .with_green(num_balls)
                .map_err(|msg| turn_parse_error_msg(game_number, turn_number, &msg))?;
        } else if color == "blue" {
            turn = turn
                .with_blue(num_balls)
                .map_err(|msg| turn_parse_error_msg(game_number, turn_number, &msg))?;
        } else {
            return Err(format!(
                "Expected color to be one of \"red\", \"green\", or \"blue\". Got {}",
                color
            ));
        }
    }
    Ok(turn)
}

fn parse_game_from_str(game_str: &str) -> Result<Game, String> {
    let mut game_turns = Vec::new();
    let (game_id, turns_to_parse) = game_str
        .split_once(": ")
        .ok_or("Expected a colon (:) separating game number and turns")?;
    let (_, game_number) = game_id
        .split_once(' ')
        .ok_or("Expected a space separating game and number")?;
    let game_number = game_number
        .parse::<usize>()
        .map_err(|_| "Failed to parse game number")?;

    let turns_to_parse = turns_to_parse.split("; ");
    for (turn_number, turn_to_parse) in turns_to_parse.enumerate() {
        game_turns.push(parse_turn_from_str(
            game_number,
            turn_number,
            turn_to_parse,
        )?);
    }

    Ok(Game {
        number: game_number,
        turns: game_turns,
    })
}

fn main() {
    let mut sum = 0;

    let ball_bag = BallBag {
        red_balls: 12,
        green_balls: 13,
        blue_balls: 14,
    };

    for (line_num, line_res) in std::io::stdin().lines().enumerate() {
        let line = line_res.unwrap_or_else(|_| panic!("LINE {}: Failed to read line!", line_num));
        let game = parse_game_from_str(&line)
            .unwrap_or_else(|_| panic!("LINE {}: Failed to parse line!", line_num));

        if game.is_possible_with_bag(&ball_bag) {
            sum += game.number;
        }
    }

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{BallColor, Game, GameTurn};

    fn game_turn(red_balls: u32, green_balls: u32, blue_balls: u32) -> GameTurn {
        let mut balls_drawn = HashMap::new();
        if red_balls > 0 {
            balls_drawn.insert(BallColor::Red, red_balls);
        }
        if green_balls > 0 {
            balls_drawn.insert(BallColor::Green, green_balls);
        }
        if blue_balls > 0 {
            balls_drawn.insert(BallColor::Blue, blue_balls);
        }
        GameTurn { balls_drawn }
    }

    fn get_test_cases() -> Vec<(String, Game)> {
        vec![
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                {
                    let turns = vec![game_turn(4, 0, 3), game_turn(1, 2, 6), game_turn(0, 2, 0)];
                    Game { number: 1, turns }
                },
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                {
                    let turns = vec![game_turn(0, 2, 1), game_turn(1, 3, 4), game_turn(0, 1, 1)];
                    Game { number: 2, turns }
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
                {
                    let turns = vec![game_turn(20, 8, 6), game_turn(4, 13, 5), game_turn(1, 5, 0)];
                    Game { number: 3, turns }
                },
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
                {
                    let turns = vec![game_turn(3, 1, 6), game_turn(6, 3, 0), game_turn(14, 3, 15)];
                    Game { number: 4, turns }
                },
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
                {
                    let turns = vec![game_turn(6, 3, 1), game_turn(1, 2, 2)];
                    Game { number: 5, turns }
                },
            ),
        ]
    }

    #[test]
    fn test_parses_game_lines_correctly() {
        let test_cases = get_test_cases();

        for (game_str, expected_game) in test_cases {
            let game = super::parse_game_from_str(&game_str).unwrap();
            assert_eq!(game, expected_game);
        }
    }

    #[test]
    fn test_is_possible_sum_is_correct() {
        let input_lines: Vec<String> = get_test_cases()
            .iter()
            .map(|(input_line, _)| input_line.to_string())
            .collect();

        let ball_bag = super::BallBag {
            red_balls: 12,
            green_balls: 13,
            blue_balls: 14,
        };

        let mut sum = 0;
        for input_line in input_lines {
            let game = super::parse_game_from_str(&input_line).unwrap();
            if game.is_possible_with_bag(&ball_bag) {
                sum += game.number;
            }
        }
        assert_eq!(8, sum);
    }
}
