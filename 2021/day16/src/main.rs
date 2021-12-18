use std::str::Chars;
use std::u8;

#[derive(Debug, PartialEq)]
enum Type {
    Literal(u64),
    OpEqual(Vec<Packet>),
    OpGreater(Vec<Packet>),
    OpLess(Vec<Packet>),
    OpMax(Vec<Packet>),
    OpMin(Vec<Packet>),
    OpProduct(Vec<Packet>),
    OpSum(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    ptype: Type,
}

impl Packet {
    fn total_version(&self) -> u32 {
        match &self.ptype {
            Type::Literal(_) => self.version.into(),
            Type::OpEqual(packets)
            | Type::OpGreater(packets)
            | Type::OpLess(packets)
            | Type::OpMax(packets)
            | Type::OpMin(packets)
            | Type::OpProduct(packets)
            | Type::OpSum(packets) => {
                self.version as u32 + packets.iter().map(|p| p.total_version()).sum::<u32>()
            }
        }
    }

    fn value(&self) -> u64 {
        match &self.ptype {
            Type::Literal(v) => *v,
            Type::OpEqual(packets) => {
                if packets[0].value() == packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Type::OpGreater(packets) => {
                if packets[0].value() > packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Type::OpLess(packets) => {
                if packets[0].value() < packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Type::OpMax(packets) => packets
                .iter()
                .map(|p| p.value())
                .max()
                .expect("Max operator packet with no subpackets"),
            Type::OpMin(packets) => packets
                .iter()
                .map(|p| p.value())
                .min()
                .expect("Min operator packet with no subpackets"),
            Type::OpProduct(packets) => packets.iter().fold(1, |acc, p| acc * p.value()),
            Type::OpSum(packets) => packets.iter().fold(0, |acc, p| acc + p.value()),
        }
    }
}

struct BinaryIter<'a> {
    chars: Chars<'a>,
    remaining: Vec<char>,
    total: usize,
}

impl<'a> BinaryIter<'a> {
    fn new(chars: Chars<'a>) -> Self {
        Self {
            chars: chars,
            remaining: vec![],
            total: 0,
        }
    }

    // ignore the remaining bits
    fn align(&mut self) {
        self.total += self.remaining.len();
        self.remaining.clear();
        // need to align by full bytes, so we might need to consume the next hex character
        // (4 bits)
        if self.total % 8 != 0 {
            self.chars.next();
            self.total += 4;
        }
    }

    // reads the next n bits and transforms them into an u64
    fn combine(&mut self, n: usize) -> Option<u64> {
        if n > 64 {
            panic!("Cannot combine more than 64-bits");
        }
        if let Some(bits) = self.group(n) {
            let v = u64::from_str_radix(&bits.into_iter().collect::<String>(), 2).unwrap();
            Some(v)
        } else {
            None
        }
    }

    // reads the next n bits
    fn group(&mut self, n: usize) -> Option<Vec<char>> {
        let mut bits: Vec<char> = vec![];
        let mut i = n;
        while i > 0 {
            if let Some(b) = self.next() {
                bits.push(b);
            }
            i -= 1;
        }
        if bits.is_empty() {
            None
        } else {
            Some(bits)
        }
    }
}

impl Iterator for BinaryIter<'_> {
    type Item = char;
    fn next(&mut self) -> Option<<Self>::Item> {
        if self.remaining.is_empty() {
            if let Some(c) = self.chars.next() {
                let v = c.to_digit(16).expect("Invalid digit");
                self.remaining.extend(format!("{:04b}", v).chars().rev());
            }
        }
        if let Some(c) = self.remaining.pop() {
            self.total += 1;
            return Some(c);
        }
        None
    }
}

struct PacketIter<'a> {
    // iter: Box<dyn Iterator<Item = &'a str>>,
    bits: BinaryIter<'a>,
    main_packet: bool,
}

impl<'a> PacketIter<'a> {
    fn new(bits: BinaryIter<'a>) -> Self {
        Self {
            bits: bits,
            main_packet: true,
        }
    }

    fn parse_literal(&mut self) -> Type {
        let mut literal_value: Vec<char> = vec![];
        while let Some(group) = self.bits.group(5) {
            literal_value.extend(&group[1..]);
            if group[0] == '0' {
                break;
            }
        }
        if self.main_packet {
            self.bits.align();
        }
        let v = u64::from_str_radix(&literal_value.into_iter().collect::<String>(), 2).unwrap();
        Type::Literal(v)
    }

    fn parse_operator(&mut self, ptype: u8) -> Type {
        let mut packets: Vec<Packet> = vec![];
        let was_main_packet = self.main_packet;
        // avoid aligning bits when parsing subpackets
        self.main_packet = false;
        match self.bits.next() {
            Some('0') => {
                let total_length = self.bits.combine(15).unwrap() as usize;
                let orig = self.bits.total;
                while self.bits.total - orig < total_length {
                    packets.push(self.next().unwrap());
                }
            }
            Some('1') => {
                let num_packets = self.bits.combine(11).unwrap();
                for _ in 0..num_packets {
                    packets.push(self.next().unwrap());
                }
            }
            Some(_) => panic!("Non-binary digit"),
            None => panic!("Invalid operator packet: missing length type ID"),
        }
        self.main_packet = was_main_packet;
        if self.main_packet {
            self.bits.align();
        }
        match ptype {
            0 => Type::OpSum(packets),
            1 => Type::OpProduct(packets),
            2 => Type::OpMin(packets),
            3 => Type::OpMax(packets),
            5 => Type::OpGreater(packets),
            6 => Type::OpLess(packets),
            7 => Type::OpEqual(packets),
            _ => panic!("Invalid packet type for Operator packet"),
        }
    }
}

impl Iterator for PacketIter<'_> {
    type Item = Packet;
    fn next(&mut self) -> Option<<Self>::Item> {
        if let Some(version) = self.bits.combine(3) {
            let ptype = match self.bits.combine(3) {
                Some(4) => self.parse_literal(),
                Some(t) => self.parse_operator(t as u8),
                _ => panic!("Invalid package type"),
            };
            return Some(Packet {
                version: version as u8,
                ptype: ptype,
            });
        }
        None
    }
}

trait Packets {
    fn packets(&self) -> PacketIter;
}

impl Packets for &str {
    fn packets(&self) -> PacketIter {
        PacketIter::new(BinaryIter::new(self.chars()))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let packets = input.trim().packets().collect::<Vec<Packet>>();
    println!(
        "Sum of all version numbers: {}",
        packets.iter().map(|p| p.total_version()).sum::<u32>()
    );
    println!("Final value: {}", packets[0].value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_iter() {
        let string = "0123456789ABCDEF";
        let iter = BinaryIter::new(string.chars());
        assert_eq!(
            iter.collect::<Vec<char>>(),
            vec![
                '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '1', '1',
                '0', '1', '0', '0', '0', '1', '0', '1', '0', '1', '1', '0', '0', '1', '1', '1',
                '1', '0', '0', '0', '1', '0', '0', '1', '1', '0', '1', '0', '1', '0', '1', '1',
                '1', '1', '0', '0', '1', '1', '0', '1', '1', '1', '1', '0', '1', '1', '1', '1'
            ]
        )
    }

    #[test]
    fn test_binary_iter_combine() {
        let string = "FFFF";
        let mut iter = BinaryIter::new(string.chars());
        assert_eq!(iter.combine(1), Some(1));
        assert_eq!(iter.combine(2), Some(3));
        assert_eq!(iter.combine(3), Some(7));
        assert_eq!(iter.combine(4), Some(15));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "D2FE28".packets().collect::<Vec<Packet>>(),
            vec![Packet {
                version: 6,
                ptype: Type::Literal(2021)
            }]
        );
        assert_eq!(
            "38006F45291200".packets().collect::<Vec<Packet>>(),
            vec![Packet {
                version: 1,
                ptype: Type::OpLess(vec![
                    Packet {
                        version: 6,
                        ptype: Type::Literal(10)
                    },
                    Packet {
                        version: 2,
                        ptype: Type::Literal(20)
                    }
                ])
            }]
        );
        assert_eq!(
            "EE00D40C823060".packets().collect::<Vec<Packet>>(),
            vec![Packet {
                version: 7,
                ptype: Type::OpMax(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1)
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2)
                    },
                    Packet {
                        version: 1,
                        ptype: Type::Literal(3)
                    }
                ])
            }]
        );
    }

    #[test]
    fn test_total_version() {
        let literal = Packet {
            version: 6,
            ptype: Type::Literal(2021),
        };
        assert_eq!(literal.total_version(), 6);
        let operator = Packet {
            version: 7,
            ptype: Type::OpSum(vec![
                Packet {
                    version: 2,
                    ptype: Type::Literal(1),
                },
                Packet {
                    version: 4,
                    ptype: Type::Literal(2),
                },
                Packet {
                    version: 1,
                    ptype: Type::Literal(3),
                },
            ]),
        };
        assert_eq!(operator.total_version(), 14);
    }

    #[test]
    fn test_value_equal() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpEqual(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                ])
            }
            .value(),
            0
        );
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpEqual(vec![
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 2,
                        ptype: Type::Literal(2),
                    },
                ])
            }
            .value(),
            1
        );
    }

    #[test]
    fn test_value_greater() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpGreater(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                ])
            }
            .value(),
            0
        );
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpGreater(vec![
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                ])
            }
            .value(),
            1
        );
    }

    #[test]
    fn test_value_less() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpLess(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                ])
            }
            .value(),
            1
        );
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpLess(vec![
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                ])
            }
            .value(),
            0
        );
    }

    #[test]
    fn test_value_max() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpMax(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 1,
                        ptype: Type::Literal(4),
                    },
                ])
            }
            .value(),
            4
        );
    }

    #[test]
    fn test_value_min() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpMin(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 1,
                        ptype: Type::Literal(4),
                    },
                ])
            }
            .value(),
            1
        );
    }

    #[test]
    fn test_value_product() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpProduct(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 1,
                        ptype: Type::Literal(4),
                    },
                ])
            }
            .value(),
            8
        );
    }

    #[test]
    fn test_value_sum() {
        assert_eq!(
            Packet {
                version: 0,
                ptype: Type::OpSum(vec![
                    Packet {
                        version: 2,
                        ptype: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        ptype: Type::Literal(2),
                    },
                    Packet {
                        version: 1,
                        ptype: Type::Literal(4),
                    },
                ])
            }
            .value(),
            7
        );
    }
}
