#[derive(Debug, PartialEq)]
struct Params {
    gamma: u32,
    epsilon: u32,
}

fn calculate_params<'a>(lines: impl Iterator<Item = &'a str>) -> Params {
    // number of 0s and 1s at each position
    let mut count0: Vec<u32> = vec![0; 32];
    let mut count1: Vec<u32> = vec![0; 32];
    for line in lines {
        for (n, b) in line.chars().rev().enumerate() {
            if n >= 32 {
                panic!("Only 32-bit integers are supported")
            }
            match b {
                '0' => count0[n] += 1,
                '1' => count1[n] += 1,
                _ => panic!("Invalid character on input: {}", b),
            }
        }
    }
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (n, (c1, c0)) in count1.iter().zip(count0).enumerate() {
        if c1 > &c0 {
            gamma += 1 << n;
        } else if c1 < &c0 {
            epsilon += 1 << n;
        }
        // if c1 == c0 then there was no input for this position n
    }
    Params {
        gamma: gamma,
        epsilon: epsilon,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let params = calculate_params(input.lines());
    let gamma = params.gamma;
    let epsilon = params.epsilon;
    println!(
        "Gamma = {} Epsilon = {} Power Consumption = {}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_param() {
        assert_eq!(
            calculate_params(
                vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ]
                .into_iter()
            ),
            Params {
                gamma: 0b10110,
                epsilon: 0b01001
            }
        );
    }
}
