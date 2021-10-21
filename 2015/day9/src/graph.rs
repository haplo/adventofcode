use itertools::Itertools;
use std::fmt::Display;

pub type Weight = i32;

pub struct Graph {
    labels: Vec<String>,
    matrix: Vec<Vec<Weight>>,
}

// Adjacency matrix undirected weighted graph
//
// Implementation relies on a given vertex V having the same index position i in both
// labels and matrix, and also that the graph is full, i.e. all vertices are
// interconnected.
impl Graph {
    pub fn new() -> Self {
        Self {
            labels: vec![],
            matrix: vec![],
        }
    }

    // add an (undirected) edge between the two given vertices
    pub fn add_edge(&mut self, label1: String, label2: String, weight: Weight) {
        let i1 = self.get_or_insert_label(label1);
        let i2 = self.get_or_insert_label(label2);
        self.matrix[i1][i2] = weight;
        self.matrix[i2][i1] = weight;
    }

    // gets matrix index for given label, insert it if not present
    fn get_or_insert_label(&mut self, label: String) -> usize {
        for (n, l) in self.labels.iter().enumerate() {
            if *l == label {
                return n;
            }
        }
        self.labels.push(label);
        for v in self.matrix.iter_mut() {
            v.push(Weight::MAX);
        }
        let n = self.matrix.len();
        let mut new_vertex = vec![Weight::MAX; n];
        new_vertex.push(0);
        self.matrix.push(new_vertex);
        n
    }

    // calculate total Weight for the given path (sequence of indexes in matrix)
    pub fn path_weight(&self, path: &Vec<usize>) -> Weight {
        let mut weight: Weight = 0;
        let mut pairs = path.windows(2);
        while let Some(&[from, to]) = pairs.next() {
            weight += self.matrix[from][to];
        }
        weight
    }

    // Find shortest path that visits all nodes
    pub fn shortest_hamiltonian_path(&self) -> Path {
        let mut best_path: Vec<usize> = vec![];
        let mut best_weight: Weight = Weight::MAX;
        let mut permutations = (0..self.matrix.len()).permutations(self.matrix.len());
        while let Some(path) = permutations.next() {
            let weight = self.path_weight(&path);
            if weight < best_weight {
                best_path = path.clone();
                best_weight = weight;
            }
        }
        Path {
            nodes: best_path.iter().map(|x| self.labels[*x].clone()).collect(),
            total_weight: best_weight,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    pub nodes: Vec<String>,
    pub total_weight: Weight,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}; Weight={}",
            self.nodes.join(" -> "),
            self.total_weight
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph: Graph = Graph::new();
        graph.add_edge("London".to_string(), "Dublin".to_string(), 464);
        graph.add_edge("London".to_string(), "Belfast".to_string(), 518);
        graph.add_edge("Dublin".to_string(), "Belfast".to_string(), 141);

        assert_eq!(
            graph.labels,
            vec![
                "London".to_string(),
                "Dublin".to_string(),
                "Belfast".to_string()
            ]
        );
        assert_eq!(
            graph.matrix,
            vec![vec![0, 464, 518], vec![464, 0, 141], vec![518, 141, 0]]
        );

        // path_weight()
        assert_eq!(graph.path_weight(&vec![0, 1, 2]), 605);
        // path_weight()
        assert_eq!(graph.path_weight(&vec![1, 2, 0]), 659);
        // path_weight()
        assert_eq!(graph.path_weight(&vec![1, 0, 2]), 982);

        // shortest_path()
        assert_eq!(
            graph.shortest_hamiltonian_path(),
            Path {
                nodes: vec![
                    "London".to_string(),
                    "Dublin".to_string(),
                    "Belfast".to_string(),
                ],
                total_weight: 605
            }
        );
    }

    #[test]
    fn test_path() {
        let path = Path {
            nodes: vec![
                "London".to_string(),
                "Dublin".to_string(),
                "Belfast".to_string(),
            ],
            total_weight: 605,
        };
        assert_eq!(
            format!("{}", path),
            "London -> Dublin -> Belfast; Weight=605"
        );
    }
}
