mod predictor {
    fn step_deltas(nums: &[i64]) -> Vec<i64> {
        assert!(nums.len() > 1);

        let mut deltas = Vec::with_capacity(nums.len() - 1);

        let mut nums_iter = nums.iter().peekable();
        let mut prev = nums_iter.next().unwrap();

        for n in nums_iter {
            deltas.push(n - prev);
            prev = n;
        }

        assert_eq!(deltas.len(), nums.len() - 1);

        deltas
    }

    fn build_delta_lists(nums: &[i64]) -> Vec<Vec<i64>> {
        let mut delta_lists = vec![nums.to_owned()];

        loop {
            let curr_list = delta_lists.last().unwrap();

            let deltas = step_deltas(curr_list.as_slice());
            let finished = deltas.iter().all(|d| *d == 0);
            delta_lists.push(deltas);

            if finished {
                break;
            }
        }
        delta_lists
    }

    fn predict_next_value(delta_lists: &[Vec<i64>]) -> i64 {
        assert!(delta_lists.len() > 1);

        let mut next = 0;

        let mut i = delta_lists.len() - 1;
        while i > 0 {
            let row = &delta_lists[i - 1];

            assert!(!row.is_empty());

            let left = row.last().unwrap();

            next += left;

            i -= 1;
        }
        next
    }

    fn predict_previous_value(delta_lists: &[Vec<i64>]) -> i64 {
        assert!(delta_lists.len() > 1);

        let mut prev = 0;

        let mut i = delta_lists.len() - 1;
        while i > 0 {
            let row = &delta_lists[i - 1];

            assert!(!row.is_empty());

            let right = row.first().unwrap();

            prev = right - prev;

            i -= 1;
        }
        prev
    }

    pub(crate) fn get_future_predictions(sensor_histories: &[Vec<i64>]) -> Vec<i64> {
        sensor_histories
            .iter()
            .map(|history| {
                let delta_lists = build_delta_lists(history);
                predict_next_value(&delta_lists)
            })
            .collect()
    }

    pub(crate) fn get_past_predictions(sensor_histories: &[Vec<i64>]) -> Vec<i64> {
        sensor_histories
            .iter()
            .map(|history| {
                let delta_lists = build_delta_lists(history);
                predict_previous_value(&delta_lists)
            })
            .collect()
    }
}
mod parse {

    pub(crate) fn parse_lists_of_numbers(
        lines: impl Iterator<Item = String>,
    ) -> Result<Vec<Vec<i64>>, String> {
        lines
            .map(|line| {
                line.split(' ')
                    .map(|num_str| {
                        num_str
                            .parse::<i64>()
                            .map_err(|e| format!("Cannot parse {} as i64: {}", num_str, e))
                    })
                    .collect()
            })
            .collect()
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let input = parse::parse_lists_of_numbers(lines).expect("Failed to parse input");
    let predictions = predictor::get_future_predictions(&input);
    println!(
        "Sum of future predictions: {}!",
        predictions.iter().sum::<i64>()
    );

    let past_predictions = predictor::get_past_predictions(&input);
    println!(
        "Sum of past predictions: {}!",
        past_predictions.iter().sum::<i64>()
    );
}

#[cfg(test)]
mod test {

    fn get_test_cases() -> Vec<(&'static str, Vec<Vec<i64>>, Vec<i64>, Vec<i64>)> {
        vec![(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
            vec![18, 28, 68],
            vec![-3, 0, 5],
        )]
    }

    #[test]
    fn test_parses_input() {
        for (input, expected, _, _) in get_test_cases() {
            let actual =
                crate::parse::parse_lists_of_numbers(input.split('\n').map(|l| l.to_string()))
                    .unwrap();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_predicts_correct_next_value() {
        for (_, sensor_histories, expected_future_predictions, _) in get_test_cases() {
            let actual_predictions = crate::predictor::get_future_predictions(&sensor_histories);

            assert_eq!(actual_predictions, expected_future_predictions);
        }
    }

    #[test]
    fn test_predicts_correct_prev_value() {
        for (_, sensor_histories, _, expected_past_predictions) in get_test_cases() {
            let actual_predictions = crate::predictor::get_past_predictions(&sensor_histories);

            assert_eq!(actual_predictions, expected_past_predictions);
        }
    }
}
