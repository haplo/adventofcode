use itertools::Itertools;

fn paper_for_present(l: u32, w: u32, h: u32) -> u32 {
    let (area1, area2, area3) = (l * w, w * h, h * l);
    let slack = {
        if area1 < area2 && area1 < area3 {
            area1
        } else if area2 < area3 {
            area2
        } else {
            area3
        }
    };
    let paper = 2 * area1 + 2 * area2 + 2 * area3 + slack;
    paper
}

fn main() {
    let mut total = 0;
    let input = std::fs::read_to_string("input.txt").unwrap();
    for dim in input.lines() {
        let (l, w, h) = dim
            .splitn(3, "x")
            .map(str::parse)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();
        total += paper_for_present(l, w, h)
    }
    println!("Need {} sq.ft. of wrapping paper", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_for_present() {
        assert_eq!(paper_for_present(2, 3, 4), 58);
        assert_eq!(paper_for_present(1, 1, 10), 43);
    }
}
