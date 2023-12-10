#[derive(Debug, PartialEq)]
pub(crate) struct RaceResult {
    time: u64,
    record_distance: u64,
}
impl RaceResult {
    pub(crate) fn new(time: u64, record_distance: u64) -> Self {
        Self {
            time,
            record_distance,
        }
    }
    pub(crate) fn ways_to_beat_record_distance(&self) -> u64 {
        let min_button_hold = 1;
        let max_button_hold = self.time - 1;

        let mut ways_to_beat_record_distance = 0;
        for i in min_button_hold..=max_button_hold {
            let time_left_to_race = self.time - i;
            let distance = time_left_to_race * i;

            if distance > self.record_distance {
                ways_to_beat_record_distance += 1;
            }
        }
        ways_to_beat_record_distance
    }
}

pub(crate) fn parse_race_results(
    mut lines: impl Iterator<Item = String>,
) -> Result<RaceResult, String> {
    let times_line = lines.next().ok_or("Expected times line")?;

    let time = times_line
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse time '{}': {}", times_line, e))?;

    let distances_line = lines.next().ok_or("Expected distances line")?;

    let distance = distances_line
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse distance '{}': {}", distances_line, e))?;

    Ok(RaceResult::new(time, distance))
}

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let race_result = parse_race_results(input).expect("Failed parsing input!");
    println!(
        "Total ways to win: {}",
        race_result.ways_to_beat_record_distance()
    );
}

#[cfg(test)]
mod test {
    use crate::{parse_race_results, RaceResult};

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_parse() {
        let expected_results = RaceResult::new(71530, 940200);

        assert_eq!(
            parse_race_results(TEST_INPUT.split("\n").into_iter().map(|s| s.to_string())).unwrap(),
            expected_results
        );
    }

    #[test]
    fn test_ways_to_win() {
        let race_result =
            parse_race_results(TEST_INPUT.split("\n").into_iter().map(|s| s.to_string())).unwrap();

        assert_eq!(race_result.ways_to_beat_record_distance(), 71503);
    }
}
