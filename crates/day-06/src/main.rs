#[derive(Debug, PartialEq)]
pub(crate) struct RaceResult {
    time: u32,
    record_distance: u32,
}
impl RaceResult {
    pub(crate) fn new(time: u32, record_distance: u32) -> Self {
        Self {
            time,
            record_distance,
        }
    }
    pub(crate) fn ways_to_beat_record_distance(&self) -> u32 {
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

pub(crate) fn total_ways_to_win(race_results: &[RaceResult]) -> u32 {
    race_results
        .iter()
        .map(|rr| rr.ways_to_beat_record_distance())
        .product()
}

pub(crate) fn parse_race_results(
    mut lines: impl Iterator<Item = String>,
) -> Result<Vec<RaceResult>, String> {
    let times_line = lines.next().ok_or_else(|| "Expected times line")?;

    let times = times_line.split_ascii_whitespace().skip(1).map(|time_str| {
        time_str
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse time '{}': {}", time_str, e))
    });

    let distances_line = lines.next().ok_or_else(|| "Expected distances line")?;

    let distances = distances_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|distance_str| {
            distance_str
                .parse::<u32>()
                .map_err(|e| format!("Failed to parse distance '{}': {}", distance_str, e))
        });

    let race_results = times
        .zip(distances)
        .map(|(time, distance)| {
            time.and_then(|time| {
                distance.map(|distance| RaceResult {
                    time,
                    record_distance: distance,
                })
            })
        })
        .collect::<Result<Vec<RaceResult>, String>>()?;

    Ok(race_results)
}

fn main() {
    let mut input = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let race_results = parse_race_results(input).expect("Failed parsing input!");
    println!("Total ways to win: {}", total_ways_to_win(&race_results));
}

#[cfg(test)]
mod test {
    use crate::{parse_race_results, RaceResult};

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_parse() {
        let expected_results = vec![
            RaceResult::new(7, 9),
            RaceResult::new(15, 40),
            RaceResult::new(30, 200),
        ];

        assert_eq!(
            parse_race_results(TEST_INPUT.split("\n").into_iter().map(|s| s.to_string())).unwrap(),
            expected_results
        );
    }

    #[test]
    fn test_ways_to_win() {
        let race_results =
            parse_race_results(TEST_INPUT.split("\n").into_iter().map(|s| s.to_string())).unwrap();

        assert_eq!(super::total_ways_to_win(&race_results), 288);
    }
}
