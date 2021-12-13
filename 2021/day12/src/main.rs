mod graph;

use graph::Graph;

const START: &str = "start";
const END: &str = "end";

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut graph = Graph::new();
    for line in input.lines() {
        let mut split = line.split("-");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        graph.add_edge(from, to);
    }
    println!(
        "Part 1: {} paths from start to end visiting small caves at most once",
        graph
            .path_iter(START.to_string(), END.to_string(), false)
            .count()
    );
    println!(
        "Part2: There are {} paths from start to end visiting a single small cave at most twice",
        graph
            .path_iter(START.to_string(), END.to_string(), true)
            .count()
    );
}
