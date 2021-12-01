fn count_increases<'a>(measures: &Vec<i32>, size: usize) -> i32 {
    let mut count = 0;
    let mut windowiter = measures.windows(size);
    let mut prev: Option<i32> = None;
    while let Some(w) = windowiter.next() {
        let current: i32 = w.iter().sum();
        if let Some(p) = prev {
            if current > p {
                count += 1;
            }
        }
        prev = Some(current);
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let measures: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let count_window_1 = count_increases(&measures, 1);
    let count_window_3 = count_increases(&measures, 3);
    println!(
        "{} measures larger than the previous measure",
        count_window_1
    );
    println!(
        "{} measures larger than the previous measure using a sliding window of 3",
        count_window_3
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        // window of size 1, for Part 1
        assert_eq!(count_increases(&Vec::<i32>::new(), 1), 0);
        assert_eq!(count_increases(&vec![1, 2, 3, 2, 2, 3, 1, 3], 1), 4);
        assert_eq!(count_increases(&vec![1, 1, 1, 1], 1), 0);
        assert_eq!(count_increases(&vec![4, 3, 2, 1], 1), 0);

        // window of size 3, for Part 2
        assert_eq!(count_increases(&Vec::<i32>::new(), 3), 0);
        assert_eq!(count_increases(&vec![1, 2, 3, 2, 2, 3, 1, 3], 3), 2);
        assert_eq!(count_increases(&vec![1, 1, 1, 1], 3), 0);
        assert_eq!(count_increases(&vec![4, 3, 2, 1], 3), 0);
    }
}
