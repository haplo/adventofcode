use std::collections::HashMap;

const STEPS_PART_1: usize = 10;
const STEPS_PART_2: usize = 40;

type Polymer = Vec<char>;
type Rules = HashMap<(char, char), char>;
type Counter = HashMap<char, u64>;

// recursive step with memoization
struct MemoStep {
    freqs: Counter,
    rules: Rules,
    memo: HashMap<(char, char, usize), Counter>,
}

impl MemoStep {
    fn new(rules: &Rules) -> Self {
        Self {
            freqs: Counter::new(),
            rules: rules.clone(),
            memo: HashMap::new(),
        }
    }

    // adds all counts from one counter into another
    fn merge_counters(from: &Counter, to: &mut Counter) {
        from.into_iter()
            .for_each(|(&c, &n)| *to.entry(c).or_insert(0) += n);
    }

    fn step(&mut self, first: char, second: char, steps: usize) -> Counter {
        let mut counter = Counter::new();
        if let Some(mem) = self.memo.get(&(first, second, steps)) {
            // memoized solution
            return mem.clone();
        } else if let Some(&i) = self.rules.get(&(first, second)) {
            // rule match
            *counter.entry(i).or_insert(0) += 1;
            if steps > 1 {
                MemoStep::merge_counters(&self.step(first, i, steps - 1), &mut counter);
                MemoStep::merge_counters(&self.step(i, second, steps - 1), &mut counter);
            }
        }
        self.memo.insert((first, second, steps), counter.clone());
        counter
    }
}

// calculate frequencies of each letter after applying the rules to the polymer over
// the given number of steps
fn step_polymer(polymer: &Polymer, rules: &Rules, steps: usize) -> Counter {
    let mut memostep = MemoStep::new(rules);
    let mut pairs = polymer.windows(2);
    while let Some(&[a, b]) = pairs.next() {
        *memostep.freqs.entry(a).or_insert(0) += 1;
        let counter = memostep.step(a, b, steps);
        MemoStep::merge_counters(&counter, &mut memostep.freqs);
    }
    // last entry in polymer is not included in the windows() iterator
    *memostep.freqs.entry(*polymer.last().unwrap()).or_insert(0) += 1;
    memostep.freqs
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let mut rules = Rules::new();
    for line in lines {
        match line.split(" -> ").collect::<Vec<&str>>()[..] {
            [pair, insert] if insert.len() == 1 => match pair.chars().collect::<Vec<char>>()[..] {
                [first, second] => rules.insert((first, second), insert.chars().next().unwrap()),
                _ => panic!("Invalid rule: {}", line),
            },
            _ => panic!("Invalid rule: {}", line),
        };
    }

    // Part 1
    let freqs = step_polymer(&polymer, &rules, STEPS_PART_1);
    let (max_char, max_count) = freqs.iter().max_by_key(|e| e.1).unwrap();
    let (min_char, min_count) = freqs.iter().min_by_key(|e| e.1).unwrap();
    println!(
        "Most common element after {} steps: {}, {} occurrences",
        STEPS_PART_1, max_char, max_count
    );
    println!(
        "Least common element after {} steps: {}, {} occurrences",
        STEPS_PART_1, min_char, min_count
    );
    println!("Solution: {}", max_count - min_count);

    // Part 2
    let freqs = step_polymer(&polymer, &rules, STEPS_PART_2);
    let (max_char, max_count) = freqs.iter().max_by_key(|e| e.1).unwrap();
    let (min_char, min_count) = freqs.iter().min_by_key(|e| e.1).unwrap();
    println!(
        "Most common element after {} steps: {}, {} occurrences",
        STEPS_PART_2, max_char, max_count
    );
    println!(
        "Least common element after {} steps: {}, {} occurrences",
        STEPS_PART_2, min_char, min_count
    );
    println!("Solution: {}", max_count - min_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_counters() {
        let mut counter = Counter::from([('A', 3), ('B', 17), ('C', 42)]);
        MemoStep::merge_counters(&Counter::new(), &mut counter);
        assert_eq!(counter, Counter::from([('A', 3), ('B', 17), ('C', 42)]));
        MemoStep::merge_counters(
            &Counter::from([('A', 2), ('B', 5), ('D', 13)]),
            &mut counter,
        );
        assert_eq!(
            counter,
            Counter::from([('A', 5), ('B', 22), ('C', 42), ('D', 13)])
        );
    }

    #[test]
    fn test_step_polymer() {
        let polymer = Polymer::from(['N', 'N', 'C', 'B']);
        let rules = Rules::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]);
        // example from Part 1
        assert_eq!(
            step_polymer(&polymer, &rules, 10),
            Counter::from([('C', 298), ('N', 865), ('B', 1749), ('H', 161)])
        );
        // example from Part 2
        let counter = step_polymer(&polymer, &rules, 40);
        assert_eq!(counter.get(&'B'), Some(&2192039569602));
        assert_eq!(counter.get(&'H'), Some(&3849876073));
    }
}
