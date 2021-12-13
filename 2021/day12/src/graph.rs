use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<String, Vec<String>>,
}

// Adjacency list undirected graph
//
// Implementation relies on a given vertex V having the same index position i in both
// labels and matrix, and also that the graph is full, i.e. all vertices are
// interconnected.
impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    // add an (undirected) edge between the two given vertices
    pub fn add_edge(&mut self, label1: &str, label2: &str) {
        self.add_simple_edge(label1.to_string(), label2.to_string());
        self.add_simple_edge(label2.to_string(), label1.to_string());
    }

    fn add_simple_edge(&mut self, from: String, to: String) {
        let edges = self.edges.entry(from).or_insert(vec![]);
        if !edges.contains(&to) {
            edges.push(to);
        }
    }

    // iterator of all paths from given start to end labels
    pub fn path_iter(&self, start: String, end: String, visit_twice: bool) -> PathIterator {
        PathIterator {
            current: Path::new(),
            start: start,
            end: end,
            graph: self,
            all_previous: HashSet::new(),
            can_visit_twice: visit_twice,
        }
    }
}

type Path = Vec<String>;

#[derive(Debug)]
pub struct PathIterator<'a> {
    current: Path,
    start: String,
    end: String,
    graph: &'a Graph,
    all_previous: HashSet<Path>,
    can_visit_twice: bool,
}

impl PathIterator<'_> {
    // true if cannot visit twice or if a small cave has already been visited twice
    fn visited_twice(&self) -> bool {
        if !self.can_visit_twice {
            return true;
        }
        for i in 2..self.current.len() {
            let prev = &self.current[i - 1];
            if *prev == prev.to_lowercase() && self.current[i..].contains(prev) {
                return true;
            }
        }
        false
    }
}

impl Iterator for PathIterator<'_> {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last;
        if self.current.is_empty() {
            // first iteration, start at the beginning
            self.current.push(self.start.clone());
            last = None;
        } else {
            // other iterations: last entry in current should be end, take it out and let
            // the algorithm find other alternatives
            last = self.current.pop();
        }
        'out: loop {
            // true if current path already visits a small cave twice or if visiting twice
            // is disallowed
            let visited_twice = self.visited_twice();
            // Find remaining edges to try. For example if last node in current path has
            // neighbors [N1, N2, N3, N4], and last is N2, then use &[N3, N4]
            let edges = self.graph.edges.get(self.current.last().unwrap()).unwrap();
            let candidates = if let Some(n) = last {
                &edges[edges.iter().position(|x| *x == n).unwrap() + 1..]
            } else {
                &edges[..]
            };
            for c in candidates {
                if c == &self.end {
                    // potential valid path, check that is hasn't been generated before
                    self.current.push(c.clone());
                    if !self.all_previous.contains(&self.current) {
                        self.all_previous.insert(self.current.clone());
                        return Some(self.current.clone());
                    }
                    self.current.pop();
                } else if *c == self.start {
                    // cannot go back to the beginning
                    continue;
                } else if *c == c.to_lowercase() && self.current.contains(c) && visited_twice {
                    // skip lowercase nodes that have already been visited in this path
                    continue;
                } else {
                    // explore this node's edges
                    self.current.push(c.clone());
                    last = None;
                    continue 'out;
                }
            }
            // no more candidates to try, step back to previous node and keep trying
            match self.current.pop() {
                Some(n) if n == self.start => return None,
                Some(n) => {
                    last = Some(n);
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_visit_once() {
        let mut graph = Graph::new();
        graph.add_edge("dc", "end");
        graph.add_edge("HN", "start");
        graph.add_edge("start", "kj");
        graph.add_edge("dc", "start");
        graph.add_edge("dc", "HN");
        graph.add_edge("LN", "dc");
        graph.add_edge("HN", "end");
        graph.add_edge("kh", "sa");
        graph.add_edge("kj", "HN");
        graph.add_edge("kj", "dc");
        assert_eq!(
            graph.edges,
            HashMap::from([
                (
                    "dc".to_string(),
                    vec![
                        "end".to_string(),
                        "start".to_string(),
                        "HN".to_string(),
                        "LN".to_string(),
                        "kj".to_string()
                    ]
                ),
                ("end".to_string(), vec!["dc".to_string(), "HN".to_string()]),
                (
                    "HN".to_string(),
                    vec![
                        "start".to_string(),
                        "dc".to_string(),
                        "end".to_string(),
                        "kj".to_string()
                    ]
                ),
                ("kh".to_string(), vec!["sa".to_string()]),
                (
                    "kj".to_string(),
                    vec!["start".to_string(), "HN".to_string(), "dc".to_string()]
                ),
                ("LN".to_string(), vec!["dc".to_string()]),
                ("sa".to_string(), vec!["kh".to_string()]),
                (
                    "start".to_string(),
                    vec!["HN".to_string(), "kj".to_string(), "dc".to_string()]
                ),
            ])
        );

        // path_iter visiting small caves at most once
        let mut path_iter = graph.path_iter("start".to_string(), "end".to_string(), false);
        assert_eq!(
            path_iter.next(),
            Some(vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "end".to_string()
            ])
        );
        assert_eq!(
            path_iter.next(),
            Some(vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string()
            ])
        );
        assert_eq!(
            path_iter.next(),
            Some(vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string()
            ])
        );
        assert_eq!(
            path_iter.next(),
            Some(vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string()
            ])
        );
        // there are 19 paths in total
        assert_eq!(path_iter.count(), 15);
    }

    #[test]
    fn test_graph_visit_twice() {
        let mut graph = Graph::new();
        graph.add_edge("fs", "end");
        graph.add_edge("he", "DX");
        graph.add_edge("fs", "he");
        graph.add_edge("start", "DX");
        graph.add_edge("pj", "DX");
        graph.add_edge("end", "zg");
        graph.add_edge("zg", "sl");
        graph.add_edge("zg", "pj");
        graph.add_edge("pj", "he");
        graph.add_edge("RW", "he");
        graph.add_edge("fs", "DX");
        graph.add_edge("pj", "RW");
        graph.add_edge("zg", "RW");
        graph.add_edge("start", "pj");
        graph.add_edge("he", "WI");
        graph.add_edge("zg", "he");
        graph.add_edge("pj", "fs");
        graph.add_edge("start", "RW");
        let path_iter = graph.path_iter("start".to_string(), "end".to_string(), true);
        assert_eq!(path_iter.count(), 3509);
    }
}
