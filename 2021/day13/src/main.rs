use std::collections::HashSet;
use std::io::{self, BufWriter, Write};

type Dot = (u32, u32);

type Paper = HashSet<Dot>;

enum Fold {
    Up(u32),
    Left(u32),
}

fn apply_fold(paper: &mut Paper, fold: &Fold) {
    let mut new = Paper::new();
    for (x, y) in paper.drain() {
        match *fold {
            Fold::Up(axis) => {
                if y < axis {
                    new.insert((x, y));
                } else if y > axis {
                    new.insert((x, y - 2 * (y - axis)));
                }
                // dots right on the folding axis are lost
            }
            Fold::Left(axis) => {
                if x < axis {
                    new.insert((x, y));
                } else if x > axis {
                    new.insert((x - 2 * (x - axis), y));
                }
                // dots right on the folding axis are lost
            }
        }
    }
    paper.extend(new);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut paper = Paper::new();
    let mut folds = Vec::<Fold>::new();
    for line in input.lines() {
        if line == "" {
            continue;
        } else if line.starts_with("fold") {
            folds.push(match line.split(" ").collect::<Vec<&str>>()[..] {
                ["fold", "along", inst] => match inst.split("=").collect::<Vec<&str>>()[..] {
                    ["x", v] => Fold::Left(v.parse::<u32>().unwrap()),
                    ["y", v] => Fold::Up(v.parse::<u32>().unwrap()),
                    _ => panic!("Invalid fold"),
                },
                _ => panic!("Invalid line"),
            });
        } else {
            let mut split = line.splitn(2, ",");
            let x = split.next().unwrap().parse::<u32>().unwrap();
            let y = split.next().unwrap().parse::<u32>().unwrap();
            paper.insert((x, y));
        }
    }
    apply_fold(&mut paper, &folds[0]);
    println!("{} dots visible after first fold", paper.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        let mut paper = Paper::from([
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ]);
        apply_fold(&mut paper, &Fold::Up(7));
        assert_eq!(
            paper,
            Paper::from([
                (0, 0),
                (0, 1),
                (0, 3),
                (1, 4),
                (2, 0),
                (3, 0),
                (3, 4),
                (4, 1),
                (4, 3),
                (6, 0),
                (6, 2),
                (6, 4),
                (8, 4),
                (9, 0),
                (9, 4),
                (10, 2),
                (10, 4),
            ])
        );
        apply_fold(&mut paper, &Fold::Left(5));
        assert_eq!(
            paper,
            Paper::from([
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 4),
                (2, 0),
                (2, 4),
                (3, 0),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ])
        );
    }
}
