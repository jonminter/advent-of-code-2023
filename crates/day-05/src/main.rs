use day5::parse::parse_input_into_mappings;

mod day5 {
    pub(crate) mod types {
        use std::ops::Deref;

        pub(crate) struct MappingForRange {
            dest_range_start: u64,
            source_range_start: u64,
            range_length: usize,
        }
        impl MappingForRange {
            pub(super) fn new(
                dest_range_start: u64,
                source_range_start: u64,
                range_length: usize,
            ) -> Self {
                Self {
                    dest_range_start,
                    source_range_start,
                    range_length,
                }
            }

            pub(crate) fn map_source_to_dest(&self, source: u64) -> Option<u64> {
                let max = self
                    .source_range_start
                    .checked_add(self.range_length as u64)
                    .expect(
                        format!(
                            "BUG: overflow {} + {}",
                            self.source_range_start, self.range_length
                        )
                        .as_str(),
                    );
                if source < self.source_range_start || source >= max {
                    return None;
                }

                let offset = source - self.source_range_start;
                let dest = self
                    .dest_range_start
                    .checked_add(offset)
                    .expect("BUG: overflow");
                Some(dest)
            }

            pub(crate) fn dest_range_start(&self) -> u64 {
                self.dest_range_start
            }

            pub(crate) fn source_range_start(&self) -> u64 {
                self.source_range_start
            }
        }

        pub(crate) struct SourceToDestMap {
            name: String,
            ranges: Vec<MappingForRange>,
        }
        impl SourceToDestMap {
            pub(super) fn new(name: String, ranges: Vec<MappingForRange>) -> Self {
                Self { name, ranges }
            }

            pub(crate) fn map_source_to_dest(&self, source: u64) -> u64 {
                for range in &self.ranges {
                    if let Some(dest) = range.map_source_to_dest(source) {
                        return dest;
                    }
                }
                source
            }

            pub(crate) fn name(&self) -> &str {
                &self.name
            }

            pub(crate) fn ranges(&self) -> &[MappingForRange] {
                &self.ranges
            }
        }

        pub(crate) struct MappingPipeline {
            mappings: Vec<SourceToDestMap>,
        }
        impl MappingPipeline {
            pub(super) fn new(mappings: Vec<SourceToDestMap>) -> Self {
                Self { mappings }
            }

            pub(crate) fn get_final_mapping(&self, seeds: &[u64]) -> Vec<u64> {
                let mut mapping = seeds.to_vec();
                for map in &self.mappings {
                    for (i, val) in mapping.iter_mut().enumerate() {
                        *val = map.map_source_to_dest(*val);
                    }
                }
                mapping
            }

            pub(crate) fn get_mapping_stages(&self) -> &[SourceToDestMap] {
                &self.mappings
            }
        }

        pub(crate) struct SeedNumbers(pub(super) Vec<u64>);
        impl Deref for SeedNumbers {
            type Target = Vec<u64>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }

    pub(crate) mod parse {
        use super::types::*;
        use std::{iter::Peekable, ops::Deref};

        struct CharAt {
            column: usize,
            c: char,
        }
        impl From<CharAt> for char {
            fn from(value: CharAt) -> Self {
                value.c
            }
        }
        impl Deref for CharAt {
            type Target = char;
            fn deref(&self) -> &Self::Target {
                &self.c
            }
        }

        fn line_char_iter(line: &str) -> Peekable<impl Iterator<Item = CharAt> + '_> {
            line.chars()
                .enumerate()
                .map(|(col, c)| CharAt { column: col, c })
                .peekable()
        }

        struct InputLine {
            line_num: usize,
            line: String,
        }
        impl InputLine {
            fn char_iter(&self) -> Peekable<impl Iterator<Item = CharAt> + '_> {
                line_char_iter(&self.line)
            }
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

            fn last_line_read(&self) -> usize {
                self.last_line_read.unwrap_or(0)
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

        fn get_error_msg_with_col(col_num: usize, msg: &str) -> String {
            format!("COL {}: {}", col_num, msg)
        }

        fn get_parse_col(
            char_iter: &mut Peekable<impl Iterator<Item = CharAt>>,
        ) -> Result<usize, String> {
            char_iter
                .peek()
                .map(|c| c.column)
                .ok_or("Empty line while getting parse col".to_string())
        }

        fn try_consume_list_of_numbers(number_list_str: &str) -> Result<Vec<u64>, String> {
            let numbers = number_list_str
                .split(" ")
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

        fn parse_mapping_range(lines: &mut InputLines) -> Result<Option<MappingForRange>, String> {
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

            Ok(Some(MappingForRange::new(
                range_numbers[0],
                range_numbers[1],
                range_numbers[2] as usize,
            )))
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

                maps.push(SourceToDestMap::new(map_name, map_ranges));

                if lines.peek().is_none() {
                    break;
                }
            }

            Ok(maps)
        }

        pub(crate) fn parse_input_into_mappings<'a>(
            lines: impl Iterator<Item = String> + 'a,
        ) -> Result<(SeedNumbers, MappingPipeline), String> {
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

            try_consume_empty_line(&mut lines)?;

            let maps = parse_maps(&mut lines)?;

            Ok((SeedNumbers(seed_numbers), MappingPipeline::new(maps)))
        }
    }
}

fn main() {
    println!("Parsing input...");
    let mut input = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let (seed_numbers, mappings) =
        parse_input_into_mappings(&mut input).expect("Failed to parse input");

    println!("Calculating final mappings...");
    let final_mapping = mappings.get_final_mapping(&seed_numbers);

    println!(
        "Lowest final mapping: {:?}",
        final_mapping.iter().min().unwrap()
    );
}

#[cfg(test)]
mod test {
    use crate::day5::parse::parse_input_into_mappings;

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
        let (seed_numbers, mapping_pipeline) = parse_input_into_mappings(line_iter).unwrap();

        assert_eq!(*seed_numbers, vec![79, 14, 55, 13]);

        assert_eq!(mapping_pipeline.get_mapping_stages().len(), 7);
    }

    #[test]
    fn test_calculates_final_mappings() {
        let line_iter = TEST_INPUT.split("\n").map(|s| s.to_string());
        let (seed_numbers, mapping_pipeline) = parse_input_into_mappings(line_iter).unwrap();

        let final_mapping = mapping_pipeline.get_final_mapping(&seed_numbers);
        assert_eq!(final_mapping, vec![82, 43, 86, 35]);
    }
}
