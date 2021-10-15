use itertools::sorted;
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

fn ribbon_for_present(l: u32, w: u32, h: u32) -> u32 {
    let (side1, side2) = sorted(vec![l, w, h].into_iter())
        .take(2)
        .collect_tuple()
        .unwrap();
    let wrap = side1 + side1 + side2 + side2;
    let bow = l * w * h;
    wrap + bow
}

fn main() {
    let mut paper = 0;
    let mut ribbon = 0;
    let input = std::fs::read_to_string("input.txt").unwrap();
    for dim in input.lines() {
        let (l, w, h) = dim
            .splitn(3, "x")
            .map(str::parse)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();
        paper += paper_for_present(l, w, h);
        ribbon += ribbon_for_present(l, w, h);
    }
    println!("Need {} sq.ft. of wrapping paper", paper);
    println!("Need {} ft. of ribbon", ribbon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_for_present() {
        assert_eq!(paper_for_present(2, 3, 4), 58);
        assert_eq!(paper_for_present(1, 1, 10), 43);
    }

    #[test]
    fn test_ribbon_for_present() {
        assert_eq!(ribbon_for_present(2, 3, 4), 34);
        assert_eq!(ribbon_for_present(2, 4, 3), 34);
        assert_eq!(ribbon_for_present(3, 2, 4), 34);
        assert_eq!(ribbon_for_present(3, 4, 2), 34);
        assert_eq!(ribbon_for_present(4, 2, 3), 34);
        assert_eq!(ribbon_for_present(4, 3, 2), 34);
        assert_eq!(ribbon_for_present(1, 1, 10), 14);
        assert_eq!(ribbon_for_present(1, 10, 1), 14);
        assert_eq!(ribbon_for_present(10, 1, 1), 14);
    }
}
