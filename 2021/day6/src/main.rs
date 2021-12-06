const RESET: u8 = 6;
const SPAWN_EXTRA: u8 = 2;
const DAYS_TO_SIMULATE: u32 = 80;

fn simulate(fishes: &mut Vec<u8>) {
    let mut spawns: u32 = 0;
    for f in fishes.iter_mut() {
        if *f == 0 {
            spawns += 1;
            *f = RESET;
        } else {
            *f -= 1;
        }
    }
    fishes.extend((0..spawns).into_iter().map(|x| RESET + SPAWN_EXTRA));
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut fishes: Vec<u8> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().expect("Expected an integer number"))
        .collect();
    for _ in 0..DAYS_TO_SIMULATE {
        simulate(&mut fishes);
    }
    println!(
        "There are {} fishes after {} days",
        fishes.len(),
        DAYS_TO_SIMULATE
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let mut fishes = vec![3, 4, 3, 1, 2];
        simulate(&mut fishes);
        assert_eq!(fishes, vec![2, 3, 2, 0, 1]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![1, 2, 1, 6, 0, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![0, 1, 0, 5, 6, 7, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![6, 0, 6, 4, 5, 6, 7, 8, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 7]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8]);
        simulate(&mut fishes);
        assert_eq!(fishes, vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8]);
    }
}
