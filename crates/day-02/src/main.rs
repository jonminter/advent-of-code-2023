use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct CubeBag {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}
impl CubeBag {
    fn power(&self) -> u32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct GameTurn {
    cubes_drawn: HashMap<CubeColor, u32>,
}
impl GameTurn {
    fn new() -> Self {
        GameTurn {
            cubes_drawn: HashMap::new(),
        }
    }

    fn with_red(mut self, red_cubes: u32) -> Result<Self, String> {
        match self.cubes_drawn.get(&CubeColor::Red) {
            None => {
                self.cubes_drawn.insert(CubeColor::Red, red_cubes);
                Ok(self)
            }
            Some(_) => Err("Already set number of red cubes drawn this turn".to_string()),
        }
    }

    fn with_green(mut self, green_cubes: u32) -> Result<Self, String> {
        match self.cubes_drawn.get(&CubeColor::Green) {
            None => {
                self.cubes_drawn.insert(CubeColor::Green, green_cubes);
                Ok(self)
            }
            Some(_) => Err("Already set number of green cubes drawn this turn".to_string()),
        }
    }

    fn with_blue(mut self, blue_cubes: u32) -> Result<Self, String> {
        match self.cubes_drawn.get(&CubeColor::Blue) {
            None => {
                self.cubes_drawn.insert(CubeColor::Blue, blue_cubes);
                Ok(self)
            }
            Some(_) => Err("Already set number of blue cubes drawn this turn".to_string()),
        }
    }

    fn num_red_drawn(&self) -> u32 {
        *self.cubes_drawn.get(&CubeColor::Red).unwrap_or(&0)
    }

    fn num_green_drawn(&self) -> u32 {
        *self.cubes_drawn.get(&CubeColor::Green).unwrap_or(&0)
    }

    fn num_blue_drawn(&self) -> u32 {
        *self.cubes_drawn.get(&CubeColor::Blue).unwrap_or(&0)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    number: usize,
    turns: Vec<GameTurn>,
}
impl Game {
    fn is_possible_with_bag(&self, bag: &CubeBag) -> bool {
        let mut max_red_drawn = 0;
        let mut max_green_drawn = 0;
        let mut max_blue_drawn = 0;
        for turn in &self.turns {
            max_red_drawn = max_red_drawn.max(turn.num_red_drawn());
            max_green_drawn = max_green_drawn.max(turn.num_green_drawn());
            max_blue_drawn = max_blue_drawn.max(turn.num_blue_drawn());
        }
        bag.red_cubes >= max_red_drawn
            && bag.green_cubes >= max_green_drawn
            && bag.blue_cubes >= max_blue_drawn
    }

    fn min_cube_bag(&self) -> CubeBag {
        let mut max_red_drawn = 0;
        let mut max_green_drawn = 0;
        let mut max_blue_drawn = 0;
        for turn in &self.turns {
            max_red_drawn = max_red_drawn.max(turn.num_red_drawn());
            max_green_drawn = max_green_drawn.max(turn.num_green_drawn());
            max_blue_drawn = max_blue_drawn.max(turn.num_blue_drawn());
        }
        CubeBag {
            red_cubes: max_red_drawn,
            green_cubes: max_green_drawn,
            blue_cubes: max_blue_drawn,
        }
    }
}

fn turn_parse_error_msg(game_num: usize, turn_num: usize, msg: &str) -> String {
    format!("GAME {}, TURN {}: {}", game_num, turn_num + 1, msg)
}

fn parse_num_cubes_and_color(
    game_number: usize,
    turn_number: usize,
    cubes_drawn: &str,
) -> Result<(u32, &str), String> {
    let (num_cubes, color) = cubes_drawn.split_once(' ').ok_or_else(|| {
        turn_parse_error_msg(
            game_number,
            turn_number,
            "Expected a space separating number of cubes and color",
        )
    })?;

    let num_cubes = num_cubes.parse::<u32>().map_err(|_| {
        turn_parse_error_msg(
            game_number,
            turn_number,
            &format!(
                "Failed to parse number of cubes in turn for color {}",
                color
            ),
        )
    })?;

    Ok((num_cubes, color))
}

fn parse_turn_from_str(
    game_number: usize,
    turn_number: usize,
    turn_to_parse: &str,
) -> Result<GameTurn, String> {
    let cubes_drawn_to_parse = turn_to_parse.split(", ");
    let mut turn = GameTurn::new();

    for cubes_drawn in cubes_drawn_to_parse {
        let (num_cubes, color) = parse_num_cubes_and_color(game_number, turn_number, cubes_drawn)?;

        if color == "red" {
            turn = turn
                .with_red(num_cubes)
                .map_err(|msg| turn_parse_error_msg(game_number, turn_number, &msg))?;
        } else if color == "green" {
            turn = turn
                .with_green(num_cubes)
                .map_err(|msg| turn_parse_error_msg(game_number, turn_number, &msg))?;
        } else if color == "blue" {
            turn = turn
                .with_blue(num_cubes)
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
    let mut sum_of_possible_games = 0;
    let mut sum_of_min_cube_bag_powers = 0;

    let bag = CubeBag {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };

    for (line_num, line_res) in std::io::stdin().lines().enumerate() {
        let line = line_res.unwrap_or_else(|_| panic!("LINE {}: Failed to read line!", line_num));
        let game = parse_game_from_str(&line)
            .unwrap_or_else(|_| panic!("LINE {}: Failed to parse line!", line_num));

        if game.is_possible_with_bag(&bag) {
            sum_of_possible_games += game.number;
        }
        sum_of_min_cube_bag_powers += game.min_cube_bag().power();
    }

    println!("Sum of possible games: {}", sum_of_possible_games);
    println!(
        "Sum of powers of min cube bags: {}",
        sum_of_min_cube_bag_powers,
    );
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{CubeColor, Game, GameTurn};

    fn game_turn(red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> GameTurn {
        let mut cubes_drawn = HashMap::new();
        if red_cubes > 0 {
            cubes_drawn.insert(CubeColor::Red, red_cubes);
        }
        if green_cubes > 0 {
            cubes_drawn.insert(CubeColor::Green, green_cubes);
        }
        if blue_cubes > 0 {
            cubes_drawn.insert(CubeColor::Blue, blue_cubes);
        }
        GameTurn { cubes_drawn }
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

        let bag = super::CubeBag {
            red_cubes: 12,
            green_cubes: 13,
            blue_cubes: 14,
        };

        let mut sum = 0;
        for input_line in input_lines {
            let game = super::parse_game_from_str(&input_line).unwrap();
            if game.is_possible_with_bag(&bag) {
                sum += game.number;
            }
        }
        assert_eq!(8, sum);
    }

    #[test]
    fn test_sum_of_powers_is_correct() {
        let input_lines: Vec<String> = get_test_cases()
            .iter()
            .map(|(input_line, _)| input_line.to_string())
            .collect();

        let mut sum = 0;
        for input_line in input_lines {
            let game = super::parse_game_from_str(&input_line).unwrap();
            sum += game.min_cube_bag().power();
        }
        assert_eq!(2286, sum);
    }
}
