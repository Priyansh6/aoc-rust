use std::collections::HashMap;

use aoc_lib::graph::{Directed, Graph};
use aoc_lib::parser;
use aoc_lib::parser::{Parser, StrParser};
use aoc_lib::solution::Solution;

fn construct_graph_and_index_map(
    devices: &Vec<(String, Vec<String>)>,
) -> (Graph<String, Directed>, HashMap<String, usize>) {
    let mut graph = Graph::new_directed();
    let mut node_to_index: HashMap<String, usize> = HashMap::new();

    for (device, outputs) in devices {
        let device_i = *node_to_index
            .entry(device.clone())
            .or_insert_with(|| graph.add_node(device.clone()));
        for output in outputs {
            let output_i = *node_to_index
                .entry(output.clone())
                .or_insert_with(|| graph.add_node(output.clone()));
            graph.add_edge(device_i, output_i, 1);
        }
    }

    (graph, node_to_index)
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<(String, Vec<String>)>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::split_pair(
            parser::as_string,
            parser::as_string.split_whitespace(),
            ": ",
        )
        .lines()
    }

    fn part1(&self, devices: &Self::Parsed) -> String {
        let (graph, node_to_index) = construct_graph_and_index_map(devices);

        let from_i = node_to_index["you"];
        let to_i = node_to_index["out"];

        graph.num_paths_dag(from_i, to_i).to_string()
    }

    fn part2(&self, devices: &Self::Parsed) -> String {
        let (graph, node_to_index) = construct_graph_and_index_map(devices);

        let from_i = node_to_index["svr"];
        let to_i = node_to_index["out"];
        let dac_i = node_to_index["dac"];
        let fft_i = node_to_index["fft"];

        graph
            .num_paths_through_dag(from_i, to_i, &[dac_i, fft_i])
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT_PART_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_PART_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT_PART_1, "5");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT_PART_2, "2");
    }
}
