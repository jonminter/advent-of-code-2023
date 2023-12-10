use std::time::SystemTime;

use day5::parse::parse_input_into_mappings;

mod day5 {
    pub(crate) mod types {
        #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
        pub(crate) struct RangeInterval(u64, u64);
        impl RangeInterval {
            pub(crate) fn new(start: u64, end: u64) -> Self {
                Self(start, end)
            }

            pub(crate) fn start(&self) -> u64 {
                self.0
            }

            pub(crate) fn end(&self) -> u64 {
                self.1
            }

            pub(crate) fn intersect(&self, other: &Self) -> Option<Self> {
                let start = self.start().max(other.start());
                let end = self.end().min(other.end());
                if start < end {
                    Some(Self(start, end))
                } else {
                    None
                }
            }
        }

        #[cfg(test)]
        mod test_range_interval {
            #[test]
            fn test_range_intersect() {
                use super::RangeInterval;
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(5, 15)),
                    Some(RangeInterval::new(5, 10))
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(10, 15)),
                    None
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(15, 20)),
                    None
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(0, 5)),
                    Some(RangeInterval::new(0, 5))
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(0, 10)),
                    Some(RangeInterval::new(0, 10))
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(0, 15)),
                    Some(RangeInterval::new(0, 10))
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(5, 10)),
                    Some(RangeInterval::new(5, 10))
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(5, 5)),
                    None
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(10, 10)),
                    None
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(10, 15)),
                    None
                );
                assert_eq!(
                    RangeInterval::new(0, 10).intersect(&RangeInterval::new(15, 15)),
                    None
                );
            }
        }

        #[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
        pub(crate) struct RangeMapping {
            source: RangeInterval,
            dest: RangeInterval,
        }

        pub(super) struct UnmappedInput(Vec<RangeInterval>);

        impl RangeMapping {
            pub(super) fn new(
                dest_range_start: u64,
                source_range_start: u64,
                range_length: usize,
            ) -> Result<Self, String> {
                let dest_range_end = dest_range_start
                    .checked_add(range_length as u64)
                    .ok_or_else(|| {
                        format!(
                            "Dest range start {} + range length {} overflows",
                            dest_range_start, range_length
                        )
                        .to_string()
                    })?;

                let source_range_end = source_range_start
                    .checked_add(range_length as u64)
                    .ok_or_else(|| {
                        format!(
                            "Source range start {} + range length {} overflows",
                            source_range_start, range_length
                        )
                        .to_string()
                    })?;

                Ok(Self {
                    source: RangeInterval(source_range_start, source_range_end),
                    dest: RangeInterval(dest_range_start, dest_range_end),
                })
            }

            pub(super) fn map_input_range(
                &self,
                input: &RangeInterval,
            ) -> (Option<RangeInterval>, UnmappedInput) {
                self.source
                    .intersect(input)
                    .map(|source_overlap| {
                        let offset = source_overlap.start() - self.source.start();
                        let output_len = source_overlap.end() - source_overlap.start();

                        let mut unmapped_input = vec![];

                        if input.start() < source_overlap.start() {
                            unmapped_input
                                .push(RangeInterval(input.start(), source_overlap.start()));
                        }

                        let mapped = RangeInterval(
                            self.dest.start() + offset,
                            self.dest.start() + offset + output_len,
                        );

                        if input.end() > source_overlap.end() {
                            unmapped_input.push(RangeInterval(source_overlap.end(), input.end()));
                        }
                        (Some(mapped), UnmappedInput(unmapped_input))
                    })
                    .unwrap_or((None, UnmappedInput(vec![input.clone()])))
            }
        }

        // Take last map, find range that has lowest destination start

        pub(crate) struct SourceToDestMap {
            _name: String,
            range_mappings: Vec<RangeMapping>,
        }
        impl SourceToDestMap {
            pub(super) fn new(
                name: String,
                mut range_mappings: Vec<RangeMapping>,
            ) -> Result<Self, String> {
                Self::check_for_overlapping_ranges(&mut range_mappings)?;
                Ok(Self {
                    _name: name,
                    range_mappings,
                })
            }

            fn check_for_overlapping_ranges(ranges: &mut Vec<RangeMapping>) -> Result<(), String> {
                ranges.sort();

                let mut prev_range_end = 0;
                for range in ranges {
                    if range.source.start() < prev_range_end {
                        return Err(format!(
                            "Overlapping ranges: {:?} starts before last range end of {:?}",
                            range, prev_range_end,
                        ));
                    }
                    prev_range_end = range.source.end();
                }
                Ok(())
            }

            pub(crate) fn map_input_range(&self, input_range: RangeInterval) -> Vec<RangeInterval> {
                let mut all_unmapped_input = vec![input_range];

                let mut mapped_output_ranges = vec![];

                for mapping in self.range_mappings.iter() {
                    let mut new_unmapped_input = vec![];
                    for range_to_map in &all_unmapped_input {
                        let (mapped_range, unmapped_input) = mapping.map_input_range(range_to_map);
                        new_unmapped_input.extend(unmapped_input.0);

                        if let Some(mapped_range) = mapped_range {
                            mapped_output_ranges.push(mapped_range);
                        }
                    }
                    all_unmapped_input = new_unmapped_input;
                }
                mapped_output_ranges.extend(all_unmapped_input);
                mapped_output_ranges
            }
        }

        pub(crate) struct MappingPipeline {
            mappings: Vec<SourceToDestMap>,
        }
        impl MappingPipeline {
            pub(super) fn new(mappings: Vec<SourceToDestMap>) -> Self {
                Self { mappings }
            }

            fn merge_intervals(ranges: Vec<RangeInterval>) -> Vec<RangeInterval> {
                let mut merged_ranges = vec![];
                let mut prev_range = ranges[0].clone();
                for range in ranges.into_iter().skip(1) {
                    if range.intersect(&prev_range).is_some() {
                        prev_range = RangeInterval(
                            prev_range.start().min(range.start()),
                            range.end().max(prev_range.end()),
                        );
                    } else {
                        merged_ranges.push(prev_range);
                        prev_range = range;
                    }
                }
                merged_ranges.push(prev_range);

                merged_ranges
            }

            pub(crate) fn get_lowest_final_mapping(
                &self,
                seeds: &[RangeInterval],
            ) -> Result<u64, String> {
                if seeds.is_empty() {
                    return Err("No seed ranges provided".to_string());
                }

                let mut lowest = u64::MAX;

                for seed_range in seeds {
                    let mut curr_ranges = vec![seed_range.clone()];
                    for mapping in self.mappings.iter() {
                        let mut new_ranges = vec![];
                        for curr_range in curr_ranges {
                            new_ranges.extend(mapping.map_input_range(curr_range));
                        }
                        new_ranges = Self::merge_intervals(new_ranges);
                        curr_ranges = new_ranges;
                    }

                    assert!(!curr_ranges.is_empty());
                    lowest = lowest.min(curr_ranges.iter().map(|r| r.start()).min().unwrap());
                }

                Ok(lowest)
            }
        }

        #[cfg(test)]
        mod test {}
    }

    pub(crate) mod parse {
        use super::types::*;
        use std::{iter::Peekable, ops::Deref};

        struct InputLine {
            line_num: usize,
            line: String,
        }
        impl Deref for InputLine {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.line
            }
        }

        struct InputLines<'a> {
            last_line_read: Option<usize>,
            iter: Peekable<Box<dyn Iterator<Item = InputLine> + 'a>>,
        }
        impl<'a> InputLines<'a> {
            fn new(lines: impl Iterator<Item = String> + 'a) -> Self {
                let iter: Box<dyn Iterator<Item = InputLine>> = Box::new(
                    lines
                        .enumerate()
                        .map(|(line_num, line)| InputLine { line_num, line }),
                );
                Self {
                    last_line_read: None,
                    iter: iter.peekable(),
                }
            }

            fn peek(&mut self) -> Option<&InputLine> {
                self.iter.peek()
            }
        }
        impl<'a> Iterator for InputLines<'a> {
            type Item = InputLine;
            fn next(&mut self) -> Option<Self::Item> {
                match self.iter.next() {
                    Some(line) => {
                        self.last_line_read = Some(line.line_num);
                        Some(line)
                    }
                    None => None,
                }
            }
        }

        fn try_consume_list_of_numbers(number_list_str: &str) -> Result<Vec<u64>, String> {
            let numbers = number_list_str
                .split(' ')
                .map(|s| s.parse::<u64>().map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, String>>()?;
            Ok(numbers)
        }

        fn try_consume_empty_line(
            line_iter: &mut impl Iterator<Item = InputLine>,
        ) -> Result<(), String> {
            let line = line_iter.next().unwrap();
            let is_empty = line.is_empty();
            if !is_empty {
                return Err(get_error_msg_with_line(
                    line.line_num,
                    "Expected empty line",
                ));
            }

            Ok(())
        }

        fn get_error_msg_with_line(line_num: usize, msg: &str) -> String {
            format!("LINE {}: {}", line_num, msg)
        }

        fn parse_mapping_name(
            lines: &mut impl Iterator<Item = InputLine>,
        ) -> Result<String, String> {
            let line = lines
                .next()
                .ok_or_else(|| "Expected mapping header".to_string())?;

            if !line.ends_with(" map:") {
                return Err(get_error_msg_with_line(
                    line.line_num,
                    &format!("Expected string ending in \"map:\", got \"{}\"", *line),
                ));
            }

            Ok(line.strip_suffix(" map:").unwrap().to_string())
        }

        fn parse_mapping_range(lines: &mut InputLines) -> Result<Option<RangeMapping>, String> {
            let maybe_line = lines.next();
            if maybe_line.is_none() {
                return Ok(None);
            }

            let line = maybe_line.unwrap();
            if line.is_empty() {
                return Ok(None);
            }

            let range_numbers = try_consume_list_of_numbers(&line)
                .map_err(|e| get_error_msg_with_line(line.line_num, &e))?;

            if range_numbers.len() != 3 {
                return Err(get_error_msg_with_line(
                    line.line_num,
                    &format!("Expected 3 numbers, got {}", range_numbers.len()),
                ));
            }

            Ok(Some(RangeMapping::new(
                range_numbers[0],
                range_numbers[1],
                range_numbers[2] as usize,
            )?))
        }

        fn parse_maps(lines: &mut InputLines) -> Result<Vec<SourceToDestMap>, String> {
            let mut maps = vec![];
            loop {
                let map_name = parse_mapping_name(lines)?;

                let mut map_ranges = vec![];
                loop {
                    match parse_mapping_range(lines) {
                        Ok(Some(range)) => map_ranges.push(range),
                        Ok(None) => break,
                        Err(e) => return Err(e),
                    }
                }

                maps.push(SourceToDestMap::new(map_name, map_ranges)?);

                if lines.peek().is_none() {
                    break;
                }
            }

            Ok(maps)
        }

        pub(crate) fn parse_input_into_mappings<'a>(
            lines: impl Iterator<Item = String> + 'a,
        ) -> Result<(Vec<RangeInterval>, MappingPipeline), String> {
            let mut lines = InputLines::new(lines);

            let line = lines
                .next()
                .ok_or_else(|| get_error_msg_with_line(0, "Unexpected end of input!"))?;
            let (header, number_list_str) = line
                .split_once(": ")
                .ok_or_else(|| "Expected header with number list".to_string())?;
            if header != "seeds" {
                return Err(get_error_msg_with_line(
                    line.line_num,
                    &format!("Expected header \"seeds\", got \"{}\"", header),
                ));
            }

            let seed_numbers = try_consume_list_of_numbers(number_list_str)
                .map_err(|e| get_error_msg_with_line(line.line_num, &e))?;
            if seed_numbers.len() % 2 != 0 {
                return Err(get_error_msg_with_line(
                    line.line_num,
                    &format!(
                        "Expected even number of seed numbers, got {}",
                        seed_numbers.len()
                    ),
                ));
            }
            let seed_ranges = seed_numbers
                .as_slice()
                .chunks(2)
                .map(|start_and_end| {
                    RangeInterval::new(start_and_end[0], start_and_end[0] + start_and_end[1])
                })
                .collect();

            try_consume_empty_line(&mut lines)?;

            let maps = parse_maps(&mut lines)?;

            Ok((seed_ranges, MappingPipeline::new(maps)))
        }
    }
}

fn main() {
    let start_time = SystemTime::now();

    println!("Parsing input...");
    let mut input = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let (seed_numbers, mappings) =
        parse_input_into_mappings(&mut input).expect("Failed to parse input");

    println!("Calculating final mappings...");
    let final_mapping = mappings.get_lowest_final_mapping(&seed_numbers);
    let end_time = SystemTime::now();
    let calc_duration = end_time.duration_since(start_time).unwrap();

    println!(
        "Lowest final mapping: {:?}; found in {:?}s and {:}ms",
        final_mapping.iter().min().unwrap(),
        calc_duration.as_secs(),
        calc_duration.subsec_millis()
    );
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use crate::day5::{parse::parse_input_into_mappings, types::RangeInterval};

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_parses_mappings() {
        let line_iter = TEST_INPUT.split("\n").map(|s| s.to_string());
        let (seed_ranges, mapping_pipeline) = parse_input_into_mappings(line_iter).unwrap();

        assert_eq!(
            *seed_ranges,
            vec![RangeInterval::new(79, 14), RangeInterval::new(55, 13)]
        );
    }

    #[test]
    fn test_calculates_final_mappings() {
        let line_iter = TEST_INPUT.split("\n").map(|s| s.to_string());
        let (seed_numbers, mapping_pipeline) = parse_input_into_mappings(line_iter).unwrap();

        println!("Seed numbers: {:?}", seed_numbers);

        let final_mapping = mapping_pipeline
            .get_lowest_final_mapping(&seed_numbers)
            .unwrap();
        assert_eq!(final_mapping, 46);
    }
}
