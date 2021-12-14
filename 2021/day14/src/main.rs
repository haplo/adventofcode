use std::collections::HashMap;
use std::str::FromStr;

const STEPS: usize = 10;

type Polymer = Vec<char>;

#[derive(Debug, PartialEq)]
struct Template {
    first: char,
    second: char,
    insert: char,
}

impl FromStr for Template {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" -> ").collect::<Vec<&str>>()[..] {
            [pair, insert] if insert.len() == 1 => match pair.chars().collect::<Vec<char>>()[..] {
                [first, second] => {
                    return Ok(Self {
                        first: first,
                        second: second,
                        insert: insert.chars().next().unwrap(),
                    })
                }
                _ => return Err(format!("Invalid pair at template: {}", s)),
            },
            _ => return Err(format!("Invalid template format: {}", s)),
        }
    }
}

fn step_polymer(polymer: &Polymer, templates: &[Template]) -> Polymer {
    let mut new = Polymer::new();
    let mut pairs = polymer.windows(2);
    while let Some(&[a, b]) = pairs.next() {
        if let Some(temp) = templates
            .iter()
            .filter(|t| t.first == a && t.second == b)
            .next()
        {
            new.extend([a, temp.insert])
        } else {
            new.push(a);
        }
    }
    new.push(*polymer.last().unwrap());
    new
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut polymer = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let templates: Vec<Template> = lines.map(|l| l.parse::<Template>().unwrap()).collect();
    for _ in 0..STEPS {
        polymer = step_polymer(&polymer, &templates);
    }

    let mut freqs: HashMap<char, u32> = HashMap::new();
    for c in polymer.iter() {
        *freqs.entry(*c).or_insert(0) += 1;
    }
    let (max_char, max_count) = freqs.iter().max_by_key(|e| e.1).unwrap();
    let (min_char, min_count) = freqs.iter().min_by_key(|e| e.1).unwrap();

    println!(
        "Most common element after {} steps: {}, {} occurrences",
        STEPS, max_char, max_count
    );
    println!(
        "Least common element after {} steps: {}, {} occurrences",
        STEPS, min_char, min_count
    );
    println!("Solution: {}", max_count - min_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        assert_eq!(
            "CH -> B".parse::<Template>(),
            Ok(Template {
                first: 'C',
                second: 'H',
                insert: 'B'
            })
        );
    }

    #[test]
    fn test_step_polymer() {
        let polymer = Polymer::from(['N', 'N', 'C', 'B']);
        let templates = vec![
            Template {
                first: 'C',
                second: 'H',
                insert: 'B',
            },
            Template {
                first: 'H',
                second: 'H',
                insert: 'N',
            },
            Template {
                first: 'C',
                second: 'B',
                insert: 'H',
            },
            Template {
                first: 'N',
                second: 'H',
                insert: 'C',
            },
            Template {
                first: 'H',
                second: 'B',
                insert: 'C',
            },
            Template {
                first: 'H',
                second: 'C',
                insert: 'B',
            },
            Template {
                first: 'H',
                second: 'N',
                insert: 'C',
            },
            Template {
                first: 'N',
                second: 'N',
                insert: 'C',
            },
            Template {
                first: 'B',
                second: 'H',
                insert: 'H',
            },
            Template {
                first: 'N',
                second: 'C',
                insert: 'B',
            },
            Template {
                first: 'N',
                second: 'B',
                insert: 'B',
            },
            Template {
                first: 'B',
                second: 'N',
                insert: 'B',
            },
            Template {
                first: 'B',
                second: 'B',
                insert: 'N',
            },
            Template {
                first: 'B',
                second: 'C',
                insert: 'B',
            },
            Template {
                first: 'C',
                second: 'C',
                insert: 'N',
            },
            Template {
                first: 'C',
                second: 'N',
                insert: 'C',
            },
        ];
        let polymer_2 = step_polymer(&polymer, &templates);
        assert_eq!(polymer_2, ['N', 'C', 'N', 'B', 'C', 'H', 'B']);
        let polymer_3 = step_polymer(&polymer_2, &templates);
        assert_eq!(
            polymer_3,
            ['N', 'B', 'C', 'C', 'N', 'B', 'B', 'B', 'C', 'B', 'H', 'C', 'B',]
        );
        let polymer_4 = step_polymer(&polymer_3, &templates);
        assert_eq!(
            polymer_4,
            [
                'N', 'B', 'B', 'B', 'C', 'N', 'C', 'C', 'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B',
                'C', 'H', 'B', 'H', 'H', 'B', 'C', 'H', 'B',
            ]
        );
        let polymer_5 = step_polymer(&polymer_4, &templates);
        assert_eq!(
            polymer_5,
            [
                'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B', 'C', 'C', 'N', 'B', 'C', 'N', 'C', 'C',
                'N', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B',
                'C', 'B', 'H', 'C', 'B', 'H', 'H', 'N', 'H', 'C', 'B', 'B', 'C', 'B', 'H', 'C',
                'B',
            ]
        );
    }
}
