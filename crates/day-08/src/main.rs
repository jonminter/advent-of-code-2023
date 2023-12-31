mod network {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) enum TraverseDir {
        Left,
        Right,
    }
    impl TraverseDir {
        pub(crate) fn from_char(c: char) -> Result<TraverseDir, String> {
            match c {
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                _ => Err("Invalid character for traverse dir!".to_string()),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct NetworkNodeEdges {
        left: String,
        right: String,
    }
    impl NetworkNodeEdges {
        pub(crate) fn new<S: Into<String>>(left: S, right: S) -> NetworkNodeEdges {
            Self {
                left: left.into(),
                right: right.into(),
            }
        }

        fn get_next_node(&self, dir: &TraverseDir) -> &str {
            use TraverseDir::*;
            match dir {
                Left => &self.left,
                Right => &self.right,
            }
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        let mut dividend = a.max(b);
        let mut divisor = a.min(b);

        loop {
            let rem = dividend % divisor;
            if rem == 0 {
                break divisor;
            }
            dividend = divisor;
            divisor = rem;
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct Network(HashMap<String, NetworkNodeEdges>);

    impl Network {
        pub(crate) fn new(graph: HashMap<String, NetworkNodeEdges>) -> Self {
            Self(graph)
        }

        fn get_node_with_label(&self, label: &str) -> Option<&NetworkNodeEdges> {
            self.0.get(label)
        }

        fn get_all_nodes_ending_in(&self, c: char) -> impl Iterator<Item = &String> {
            self.0.keys().filter(move |n| n.ends_with(c))
        }

        fn find_num_steps_till_z(&self, start_node: &str, instructions: &[TraverseDir]) -> usize {
            let mut instructions_iter = instructions.iter().cycle();
            let mut num_steps = 0;
            let mut curr_node = start_node;

            loop {
                if curr_node.ends_with('Z') {
                    break num_steps;
                }
                let next_dir = instructions_iter.next().unwrap();
                curr_node = self
                    .get_node_with_label(curr_node)
                    .expect("BUG: Node should exist")
                    .get_next_node(next_dir);
                num_steps += 1;
            }
        }

        pub(crate) fn num_steps_till_all_a_nodes_end_in_z(
            &self,
            instructions: &[TraverseDir],
        ) -> usize {
            self.get_all_nodes_ending_in('A')
                .map(|n| self.find_num_steps_till_z(n, instructions))
                .fold(1, |result, num_steps| lcm(num_steps, result))
        }

        pub(crate) fn num_steps_to_zzz(&self, instructions: &[TraverseDir]) -> usize {
            self.find_num_steps_till_z("AAA", instructions)
        }
    }
}

mod parse {
    use std::collections::HashMap;

    use crate::network::{Network, NetworkNodeEdges, TraverseDir};

    fn parse_instructions(line: &str) -> Result<Vec<TraverseDir>, String> {
        line.chars().map(TraverseDir::from_char).collect()
    }

    fn parse_network(lines: impl Iterator<Item = String>) -> Result<Network, String> {
        let graph = lines
            .map(|line| {
                let (label, mut edges) = line.split_once('=').ok_or("Missing '='".to_string())?;
                edges = edges.trim();
                edges = edges
                    .strip_prefix('(')
                    .ok_or("Expected edges to start with '('".to_string())?;
                edges = edges
                    .strip_suffix(')')
                    .ok_or("Expected edges to end with ')'")?;
                let (left, right) = edges.split_once(',').ok_or("Missing ',' between edges")?;
                Ok((
                    label.trim().to_string(),
                    NetworkNodeEdges::new(left.trim(), right.trim()),
                ))
            })
            .collect::<Result<HashMap<_, _>, String>>()?;
        Ok(Network::new(graph))
    }

    pub(crate) fn parse_input_as_network_and_instructions(
        mut lines: impl Iterator<Item = String>,
    ) -> Result<(Network, Vec<TraverseDir>), String> {
        let instructions_line = lines.next().ok_or("Unexpected empty input!")?;
        let instructions = parse_instructions(&instructions_line)?;

        lines
            .next()
            .ok_or("Unexpected end of input!".to_string())
            .and_then(|l| {
                if l.is_empty() {
                    Ok(())
                } else {
                    Err("Expected empty line!".to_string())
                }
            })?;

        let network = parse_network(lines)?;

        Ok((network, instructions))
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|s| s.expect("Failed to read line"));

    let (network, instructions) =
        parse::parse_input_as_network_and_instructions(lines).expect("Failed parsing input");

    println!("Part 1 = {:?}", network.num_steps_to_zzz(&instructions));
    let num_steps = network.num_steps_till_all_a_nodes_end_in_z(&instructions);
    println!("Num steps = {:?}", num_steps);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{network::*, parse::parse_input_as_network_and_instructions};

    use super::*;

    struct TestCase {
        input: &'static str,
        expected_network: network::Network,
        expected_instructions: Vec<TraverseDir>,
        expected_steps: usize,
    }
    impl TestCase {
        fn line_iter(&self) -> impl Iterator<Item = String> {
            self.input.split('\n').map(|s| s.to_string())
        }
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                input: r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
                expected_network: {
                    let mut network_graph = HashMap::new();
                    network_graph.insert("AAA".to_string(), NetworkNodeEdges::new("BBB", "CCC"));
                    network_graph.insert("BBB".to_string(), NetworkNodeEdges::new("DDD", "EEE"));
                    network_graph.insert("CCC".to_string(), NetworkNodeEdges::new("ZZZ", "GGG"));
                    network_graph.insert("DDD".to_string(), NetworkNodeEdges::new("DDD", "DDD"));
                    network_graph.insert("EEE".to_string(), NetworkNodeEdges::new("EEE", "EEE"));
                    network_graph.insert("GGG".to_string(), NetworkNodeEdges::new("GGG", "GGG"));
                    network_graph.insert("ZZZ".to_string(), NetworkNodeEdges::new("ZZZ", "ZZZ"));

                    Network::new(network_graph)
                },
                expected_instructions: vec![TraverseDir::Right, TraverseDir::Left],
                expected_steps: 2,
            },
            TestCase {
                input: r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
                expected_network: {
                    let mut network_graph = HashMap::new();
                    network_graph.insert("AAA".to_string(), NetworkNodeEdges::new("BBB", "BBB"));
                    network_graph.insert("BBB".to_string(), NetworkNodeEdges::new("AAA", "ZZZ"));
                    network_graph.insert("ZZZ".to_string(), NetworkNodeEdges::new("ZZZ", "ZZZ"));

                    Network::new(network_graph)
                },
                expected_instructions: vec![
                    TraverseDir::Left,
                    TraverseDir::Left,
                    TraverseDir::Right,
                ],
                expected_steps: 6,
            },
        ]
    }

    #[test]
    fn test_parse() {
        let test_cases = get_test_cases();

        for test_case in test_cases {
            let (network, instructions) =
                parse_input_as_network_and_instructions(test_case.line_iter()).unwrap();

            assert_eq!(network, test_case.expected_network);
            assert_eq!(instructions, test_case.expected_instructions);
        }
    }

    #[test]
    fn test_calculates_steps_correctly() {
        let test_cases = get_test_cases();

        for test_case in test_cases {
            let (network, instructions) =
                parse_input_as_network_and_instructions(test_case.line_iter()).unwrap();

            assert_eq!(
                network.num_steps_to_zzz(&instructions),
                test_case.expected_steps
            );
        }
    }

    #[test]
    fn test_calculates_steps_multi_start_nodes() {
        let test_input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let (network, instructions) =
            parse_input_as_network_and_instructions(test_input.split("\n").map(|l| l.to_string()))
                .unwrap();

        let steps = network.num_steps_till_all_a_nodes_end_in_z(&instructions);

        assert_eq!(steps, 6);
    }
}
