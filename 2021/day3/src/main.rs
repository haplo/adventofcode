type Counts = Vec<(u32, u32)>;

#[derive(Debug)]
struct Diagnostics {
    number_len: usize,       // number of digits on each input number
    numbers: Vec<Vec<char>>, // input numbers as vector of characters
    counts: Counts,          // number of 0s and 1s at each position
}

#[derive(Debug)]
enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn count<'a>(numbers: &Vec<&'a Vec<char>>, len: usize) -> Counts {
    let mut counts: Counts = vec![(0, 0); len];
    for num in numbers.iter() {
        for (n, b) in num.iter().enumerate() {
            if n >= 32 {
                panic!("Only 32-bit integers are supported")
            }
            match b {
                '0' => counts[n].0 += 1,
                '1' => counts[n].1 += 1,
                _ => panic!("Invalid character on input: {}", b),
            }
        }
    }
    counts
}

fn parse_diagnostics(input: &String) -> Diagnostics {
    let mut number_len = None;
    let mut numbers = vec![];
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        if number_len == None {
            number_len = Some(chars.len());
        } else if number_len != Some(chars.len()) {
            panic!("All diagnostic numbers must have the same length");
        }
        numbers.push(chars);
    }
    let number_len = number_len.unwrap_or(0);
    let counts = count(&numbers.iter().collect(), number_len);
    Diagnostics {
        number_len: number_len,
        numbers: numbers,
        counts: counts,
    }
}

fn calculate_rate(diag: &Diagnostics, criteria: BitCriteria) -> u32 {
    let mut rate: u32 = 0;
    for (n, (c0, c1)) in diag.counts.iter().rev().enumerate() {
        match criteria {
            BitCriteria::MostCommon => {
                if c1 > c0 {
                    rate += 1 << n;
                }
            }
            BitCriteria::LeastCommon => {
                if c1 < c0 {
                    rate += 1 << n;
                }
            }
        }
    }
    rate
}

fn calculate_rating(diag: &Diagnostics, criteria: BitCriteria) -> u32 {
    let mut candidates: Vec<&Vec<char>> = diag.numbers.iter().collect();
    let mut pos = 0;
    while candidates.len() > 1 {
        if pos >= diag.number_len {
            panic!("Too many candidates");
        }
        let counts = count(&candidates, diag.number_len);
        let (count0, count1) = counts[pos];
        let target = match criteria {
            BitCriteria::MostCommon => {
                if count0 > count1 {
                    '0'
                } else {
                    '1'
                }
            }
            BitCriteria::LeastCommon => {
                if count0 > count1 {
                    '1'
                } else {
                    '0'
                }
            }
        };
        candidates = candidates
            .into_iter()
            .filter(|c| c[pos] == target)
            .collect();
        pos += 1;
    }
    u32::from_str_radix(&candidates[0].into_iter().collect::<String>(), 2).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let diag = parse_diagnostics(&input);
    let gamma = calculate_rate(&diag, BitCriteria::MostCommon);
    let epsilon = calculate_rate(&diag, BitCriteria::LeastCommon);
    println!(
        "Gamma = {} Epsilon = {} Power Consumption = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    let o2 = calculate_rating(&diag, BitCriteria::MostCommon);
    let co2 = calculate_rating(&diag, BitCriteria::LeastCommon);
    println!(
        "Oxygen Generator Rating = {} CO2 Scrubber Rating = {} Life Support Rating = {}",
        o2,
        co2,
        o2 * co2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_calculate_gamma_epsilon() {
        let diag = &parse_diagnostics(&SAMPLE_INPUT.to_string());
        assert_eq!(calculate_rate(&diag, BitCriteria::MostCommon), 0b10110);
        assert_eq!(calculate_rate(&diag, BitCriteria::LeastCommon), 0b01001);
    }

    #[test]
    fn test_calculate_rating() {
        let diag = &parse_diagnostics(&SAMPLE_INPUT.to_string());
        assert_eq!(calculate_rating(&diag, BitCriteria::MostCommon), 0b10111);
        assert_eq!(calculate_rating(&diag, BitCriteria::LeastCommon), 0b01010);
    }
}
