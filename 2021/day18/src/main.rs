use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Num {
    Simple(u8),
    Pair(Box<SnailNum>),
}

impl Num {
    // add v to the leftmost Simple value
    fn add_left(&mut self, v: u8) {
        match self {
            Num::Simple(n) => *n += v,
            Num::Pair(p) => p.left.add_left(v),
        }
    }

    // add v to the rightmost Simple value
    fn add_right(&mut self, v: u8) {
        match self {
            Num::Simple(n) => *n += v,
            Num::Pair(p) => p.right.add_right(v),
        }
    }

    // true when a split operation happens
    fn split(&mut self) -> bool {
        match self {
            Num::Simple(n) if *n >= 10 => {
                *self = Num::Pair(Box::new(SnailNum {
                    left: Num::Simple(*n / 2),
                    right: Num::Simple(if *n % 2 == 0 { *n / 2 } else { (*n + 1) / 2 }),
                }));
                true
            }
            Num::Pair(n) => n.left.split() || n.right.split(),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SnailNum {
    left: Num,
    right: Num,
}

impl SnailNum {
    fn new(a: SnailNum, b: SnailNum) -> Self {
        Self {
            left: Num::Pair(Box::new(a)),
            right: Num::Pair(Box::new(b)),
        }
    }

    fn _explode(&mut self, depth: usize) -> (bool, Option<u8>, Option<u8>) {
        let mut exploded = false;
        if depth == 5 {
            match (&self.left, &self.right) {
                (Num::Simple(l), Num::Simple(r)) => return (true, Some(*l), Some(*r)),
                _ => panic!("Bad SnailNum more than 5 levels deep!"),
            }
        }
        if let Num::Pair(n) = &mut self.left {
            let (left_exploded, l, r) = n._explode(depth + 1);
            exploded |= left_exploded;
            if left_exploded {
                match (l, r) {
                    (Some(l), Some(r)) => {
                        // left value just exploded, replace with 0 and add its values
                        self.left = Num::Simple(0);
                        self.right.add_left(r);
                        return (true, Some(l), None);
                    }
                    (Some(l), None) => {
                        return (true, Some(l), None);
                    }
                    (None, Some(r)) => {
                        self.right.add_left(r);
                        return (true, None, None);
                    }
                    (None, None) => {
                        return (true, None, None);
                    }
                };
            }
        }
        if !exploded {
            if let Num::Pair(n) = &mut self.right {
                let (right_exploded, l, r) = n._explode(depth + 1);
                exploded |= right_exploded;
                match (l, r) {
                    (Some(l), Some(r)) => {
                        // right value just exploded, replace with 0 and add its values
                        self.right = Num::Simple(0);
                        self.left.add_right(l);
                        return (exploded, None, Some(r));
                    }
                    (Some(l), None) => {
                        self.left.add_right(l);
                        return (exploded, None, None);
                    }
                    (None, Some(r)) => {
                        return (exploded, None, Some(r));
                    }
                    (None, None) => return (exploded, None, None),
                }
            }
        }
        (exploded, None, None)
    }

    // true if one pair exploded, false otherwise
    fn explode(&mut self) -> bool {
        let (exploded, _, _) = self._explode(1);
        exploded
    }

    fn split(&mut self) -> bool {
        if self.left.split() {
            return true;
        } else if self.right.split() {
            return true;
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                // exploded, try exploding again as only one pair explodes each call
                continue;
            }
            if self.split() {
                // after a split need to try exploding again
                continue;
            }
            break;
        }
    }
}

fn parse_num(chars: &mut Peekable<Chars>) -> Num {
    let c = *chars.peek().unwrap();
    if c == '[' {
        Num::Pair(Box::new(parse_snailnum(chars)))
    } else {
        let mut digits = String::new();
        loop {
            let c = chars.peek().unwrap();
            if c.is_digit(10) {
                digits.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        Num::Simple(digits.parse::<u8>().expect("Bad number"))
    }
}

fn parse_snailnum(chars: &mut Peekable<Chars>) -> SnailNum {
    let c = chars.next().unwrap();
    if c != '[' {
        panic!("Expected a [, found {}", c);
    }
    let left = parse_num(chars);
    let c = chars.next().expect("Missing comma");
    if c != ',' {
        panic!("Expected a comma, found {}", c);
    }
    let right = parse_num(chars);
    let c = chars.next().unwrap();
    if c != ']' {
        panic!("Expected a ], found {}", c);
    }
    SnailNum {
        left: left,
        right: right,
    }
}

impl FromStr for SnailNum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_snailnum(&mut s.chars().peekable()))
    }
}

impl Add for &SnailNum {
    type Output = SnailNum;

    fn add(self, rhs: Self) -> SnailNum {
        let mut n = SnailNum::new(self.clone(), rhs.clone());
        n.reduce();
        n
    }
}

impl Add for SnailNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut n = SnailNum::new(self, rhs);
        n.reduce();
        n
    }
}

trait Magnitude {
    fn magnitude(&self) -> u64;
}

impl Magnitude for Num {
    fn magnitude(&self) -> u64 {
        match self {
            Num::Simple(v) => *v as u64,
            Num::Pair(p) => p.magnitude(),
        }
    }
}

impl Magnitude for SnailNum {
    fn magnitude(&self) -> u64 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

fn find_largest_sum(nums: Vec<SnailNum>) -> u64 {
    let mut best: u64 = u64::MIN;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let m = (&nums[i] + &nums[j]).magnitude();
            if m > best {
                best = m;
            }
        }
    }
    best
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let nums: Vec<SnailNum> = input
        .lines()
        .map(|l| l.parse::<SnailNum>().unwrap())
        .collect();
    println!(
        "Magnitude of final result: {}",
        nums.clone()
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap()
            .magnitude()
    );
    println!(
        "Largest magnitude of adding a pair: {}",
        find_largest_sum(nums)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use Num::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "[1,2]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Simple(1),
                right: Simple(2)
            })
        );
        assert_eq!(
            "[[1,2],3]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Pair(Box::new(SnailNum {
                    left: Simple(1),
                    right: Simple(2)
                })),
                right: Simple(3)
            })
        );
        assert_eq!(
            "[9,[8,7]]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Simple(9),
                right: Pair(Box::new(SnailNum {
                    left: Simple(8),
                    right: Simple(7)
                })),
            })
        );
        assert_eq!(
            "[[1,9],[8,5]]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Pair(Box::new(SnailNum {
                    left: Simple(1),
                    right: Simple(9)
                })),
                right: Pair(Box::new(SnailNum {
                    left: Simple(8),
                    right: Simple(5)
                }))
            })
        );
        assert_eq!(
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Pair(Box::new(SnailNum {
                    left: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(1),
                            right: Simple(2)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(3),
                            right: Simple(4)
                        }))
                    })),
                    right: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(5),
                            right: Simple(6)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(7),
                            right: Simple(8)
                        }))
                    }))
                })),
                right: Simple(9)
            })
        );
        assert_eq!(
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".parse::<SnailNum>(),
            Ok(SnailNum {
                left: Pair(Box::new(SnailNum {
                    left: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(1),
                            right: Simple(3)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(5),
                            right: Simple(3)
                        }))
                    })),
                    right: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(1),
                            right: Simple(3)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(8),
                            right: Simple(7)
                        }))
                    }))
                })),
                right: Pair(Box::new(SnailNum {
                    left: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(4),
                            right: Simple(9)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(6),
                            right: Simple(9)
                        }))
                    })),
                    right: Pair(Box::new(SnailNum {
                        left: Pair(Box::new(SnailNum {
                            left: Simple(8),
                            right: Simple(2)
                        })),
                        right: Pair(Box::new(SnailNum {
                            left: Simple(7),
                            right: Simple(3)
                        }))
                    }))
                }))
            })
        );
    }

    #[test]
    fn test_explode() {
        let mut num = "[[[[[9,8],1],2],3],4]".parse::<SnailNum>().unwrap();
        num.explode();
        assert_eq!(num, "[[[[0,9],2],3],4]".parse::<SnailNum>().unwrap());

        let mut num = "[7,[6,[5,[4,[3,2]]]]]".parse::<SnailNum>().unwrap();
        num.explode();
        assert_eq!(num, "[7,[6,[5,[7,0]]]]".parse::<SnailNum>().unwrap());

        let mut num = "[[6,[5,[4,[3,2]]]],1]".parse::<SnailNum>().unwrap();
        num.explode();
        assert_eq!(num, "[[6,[5,[7,0]]],3]".parse::<SnailNum>().unwrap());

        let mut num = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<SnailNum>()
            .unwrap();
        num.explode();
        assert_eq!(
            num,
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );

        let mut num = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<SnailNum>()
            .unwrap();
        num.explode();
        assert_eq!(
            num,
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse::<SnailNum>().unwrap()
        );

        let mut num = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<SnailNum>()
            .unwrap();
        num.explode();
        assert_eq!(
            num,
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num.explode();
        assert_eq!(
            num,
            "[[[[0,7],4],[15,[0,13]]],[1,1]]"
                .parse::<SnailNum>()
                .unwrap()
        );
    }

    #[test]
    fn test_num_split() {
        let mut num = Num::Simple(9);
        assert_eq!(num.split(), false);

        let mut num = Num::Simple(15);
        assert_eq!(num.split(), true);
        assert_eq!(
            num,
            Num::Pair(Box::new(SnailNum {
                left: Num::Simple(7),
                right: Num::Simple(8),
            }))
        );

        let mut num = Num::Pair(Box::new(SnailNum {
            left: Num::Simple(7),
            right: Num::Simple(8),
        }));
        assert_eq!(num.split(), false);

        let mut num = Num::Pair(Box::new(SnailNum {
            left: Num::Simple(14),
            right: Num::Simple(8),
        }));
        assert_eq!(num.split(), true);
        assert_eq!(
            num,
            Num::Pair(Box::new(SnailNum {
                left: Num::Pair(Box::new(SnailNum {
                    left: Num::Simple(7),
                    right: Num::Simple(7),
                })),
                right: Num::Simple(8),
            }))
        );
    }

    #[test]
    fn test_snailnum_split() {
        let mut num = SnailNum {
            left: Num::Pair(Box::new(SnailNum {
                left: Num::Simple(14),
                right: Num::Simple(8),
            })),
            right: Num::Pair(Box::new(SnailNum {
                left: Num::Simple(6),
                right: Num::Simple(12),
            })),
        };
        assert_eq!(num.split(), true);
        assert_eq!(
            num,
            SnailNum {
                left: Num::Pair(Box::new(SnailNum {
                    left: Num::Pair(Box::new(SnailNum {
                        left: Num::Simple(7),
                        right: Num::Simple(7),
                    })),
                    right: Num::Simple(8),
                })),
                right: Num::Pair(Box::new(SnailNum {
                    left: Num::Simple(6),
                    right: Num::Simple(12),
                })),
            }
        );
        assert_eq!(num.split(), true);
        let prev = num.clone();
        assert_eq!(
            num,
            SnailNum {
                left: Num::Pair(Box::new(SnailNum {
                    left: Num::Pair(Box::new(SnailNum {
                        left: Num::Simple(7),
                        right: Num::Simple(7),
                    })),
                    right: Num::Simple(8),
                })),
                right: Num::Pair(Box::new(SnailNum {
                    left: Num::Simple(6),
                    right: Num::Pair(Box::new(SnailNum {
                        left: Num::Simple(6),
                        right: Num::Simple(6),
                    })),
                })),
            }
        );
        assert_eq!(num.split(), false);
        assert_eq!(num, prev);
    }

    #[test]
    fn test_reduce() {
        let mut num = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
            .parse::<SnailNum>()
            .unwrap();
        num.reduce();
        assert_eq!(
            num,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SnailNum>()
                .unwrap()
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            Vec::from(["[1,1]", "[2,2]", "[3,3]", "[4,4]",])
                .iter()
                .map(|s| s.parse::<SnailNum>().unwrap())
                .reduce(|a, b| a + b),
            Some("[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<SnailNum>().unwrap())
        );
        assert_eq!(
            Vec::from(["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]",])
                .iter()
                .map(|s| s.parse::<SnailNum>().unwrap())
                .reduce(|a, b| a + b),
            Some("[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<SnailNum>().unwrap())
        );
        assert_eq!(
            Vec::from(["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]",])
                .iter()
                .map(|s| s.parse::<SnailNum>().unwrap())
                .reduce(|a, b| a + b),
            Some("[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<SnailNum>().unwrap())
        );
        // larger example, step by step
        let mut num = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<SnailNum>()
            .unwrap();
        num = num
            + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
                .parse::<SnailNum>()
                .unwrap();
        assert_eq!(
            num,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num
            + "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"
                .parse::<SnailNum>()
                .unwrap();
        // [[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[0,[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[5,0],[[11,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[5,11],[0,[13,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[5,11],[13,0]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[5,11],[13,0]],[[15,0],[[14,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        // [[[[5,11],[13,0]],[[15,14],[0,[14,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]
        assert_eq!(
            num,
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num
            + "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
                .parse::<SnailNum>()
                .unwrap();
        assert_eq!(
            num,
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[7,[5,[[3,8],[1,4]]]]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[[2,[2,2]],[8,[8,1]]]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[2,9]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[[[5,[7,4]],7],1]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        num = num + "[[[[4,2],2],6],[8,7]]".parse::<SnailNum>().unwrap();
        assert_eq!(
            num,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailNum>()
                .unwrap()
        );
        assert_eq!(
            Vec::from([
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                "[[[[5,4],[7,7]],8],[[8,3],8]]",
                "[[9,3],[[9,9],[6,[4,9]]]]",
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ])
            .iter()
            .map(|s| s.parse::<SnailNum>().unwrap())
            .reduce(|a, b| a + b),
            Some(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
                    .parse::<SnailNum>()
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            "[[1,2],[[3,4],5]]".parse::<SnailNum>().unwrap().magnitude(),
            143
        );
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SnailNum>()
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<SnailNum>()
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<SnailNum>()
                .unwrap()
                .magnitude(),
            791
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<SnailNum>()
                .unwrap()
                .magnitude(),
            1137
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailNum>()
                .unwrap()
                .magnitude(),
            3488
        );
    }

    #[test]
    fn test_find_largest_sum() {
        let nums: Vec<SnailNum> = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .into_iter()
        .map(|s| s.parse::<SnailNum>().unwrap())
        .collect();
        assert_eq!(find_largest_sum(nums), 3993);
    }
}
