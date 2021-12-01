fn count_increases(measures: impl Iterator<Item = i32>) -> i32 {
    let mut peekiter = measures.peekable();
    let mut count = 0;
    while let Some(m) = peekiter.next() {
        if let Some(&second) = peekiter.peek() {
            if second > m {
                count += 1
            }
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let measures = input.lines().map(|l| l.parse::<i32>().unwrap());
    let count = count_increases(measures);
    println!("{} measures larger than the previous measure", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases([].into_iter()), 0);
        assert_eq!(count_increases(vec![1, 2, 3, 2, 2, 3, 1, 3].into_iter()), 4);
        assert_eq!(count_increases(vec![1, 1, 1, 1].into_iter()), 0);
        assert_eq!(count_increases(vec![4, 3, 2, 1].into_iter()), 0);
    }
}
