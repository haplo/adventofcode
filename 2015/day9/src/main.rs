mod graph;

use graph::{Graph, Mode, Weight};

#[derive(Debug, Eq, PartialEq)]
struct ParsedLine {
    from: String,
    to: String,
    weight: i32,
}

fn parse_line(line: &str) -> ParsedLine {
    match line.split(" ").collect::<Vec<&str>>()[..] {
        [from, "to", to, "=", weight] => ParsedLine {
            from: from.to_string(),
            to: to.to_string(),
            weight: weight.parse::<Weight>().expect("Bad weight"),
        },
        _ => panic!("Bad line format"),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut graph: Graph = Graph::new();
    for line in input.lines() {
        let parsed_line = parse_line(line);
        graph.add_edge(parsed_line.from, parsed_line.to, parsed_line.weight);
    }
    let shortest_path = graph.hamiltonian_path(Mode::Shortest);
    println!("Shortest path: {}", shortest_path);
    let longest_path = graph.hamiltonian_path(Mode::Longest);
    println!("Shortest path: {}", longest_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line(&"London to Dublin = 464"),
            ParsedLine {
                from: "London".to_string(),
                to: "Dublin".to_string(),
                weight: 464
            }
        )
    }
}
