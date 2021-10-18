use std::collections::HashMap;
use std::str::FromStr;

type WireName = String;
type State = HashMap<WireName, i32>;

#[derive(Debug, Eq, PartialEq)]
enum ValueOrWire {
    Wire(WireName),
    Value(i32),
}

impl ValueOrWire {
    fn input_ready(&self, state: &State) -> bool {
        match self {
            ValueOrWire::Value(_) => true,
            ValueOrWire::Wire(w) if state.contains_key(w) => true,
            _ => false,
        }
    }

    fn value(&self, state: &State) -> Result<i32, &str> {
        match self {
            ValueOrWire::Value(n) => Ok(*n),
            ValueOrWire::Wire(w) => {
                if let Some(v) = state.get(w) {
                    Ok(*v)
                } else {
                    Err("Input wire not in current state")
                }
            }
        }
    }
}

impl FromStr for ValueOrWire {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match s.parse::<i32>() {
            Ok(n) => Ok(ValueOrWire::Value(n)),
            Err(_) => Ok(ValueOrWire::Wire(s.to_string())),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Op1 {
    source: ValueOrWire,
    target: WireName,
}

#[derive(Debug, Eq, PartialEq)]
struct Op2 {
    source1: ValueOrWire,
    source2: ValueOrWire,
    target: WireName,
}

#[derive(Debug, Eq, PartialEq)]
struct OpShift {
    source: ValueOrWire,
    amount: i32,
    target: WireName,
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Assign(Op1),
    And(Op2),
    Or(Op2),
    Not(Op1),
    LShift(OpShift),
    RShift(OpShift),
}

trait Executable {
    fn execute(&self, state: &mut State) -> Result<(), &str>;
    fn is_executable(&self, state: &State) -> bool;
}

impl Executable for Operation {
    fn execute(&self, state: &mut State) -> Result<(), &str> {
        if !self.is_executable(state) {
            return Err("Operation is not executable: not all inputs are ready");
        }
        match self {
            Operation::Assign(op) => {
                state.insert(op.target.clone(), op.source.value(state)?);
                Ok(())
            }
            Operation::And(op) => {
                state.insert(
                    op.target.clone(),
                    op.source1.value(state)? & op.source2.value(state)?,
                );
                Ok(())
            }
            Operation::Or(op) => {
                state.insert(
                    op.target.clone(),
                    op.source1.value(state)? | op.source2.value(state)?,
                );
                Ok(())
            }
            Operation::Not(op) => {
                state.insert(op.target.clone(), !op.source.value(state)?);
                Ok(())
            }
            Operation::LShift(op) => {
                state.insert(op.target.clone(), op.source.value(state)? << op.amount);
                Ok(())
            }
            Operation::RShift(op) => {
                state.insert(op.target.clone(), op.source.value(state)? >> op.amount);
                Ok(())
            }
        }
    }

    // return true if can be executed, false if inputs are missing
    fn is_executable(&self, state: &State) -> bool {
        match self {
            Operation::Assign(op) | Operation::Not(op) => op.source.input_ready(state),
            Operation::And(op) | Operation::Or(op) => {
                op.source1.input_ready(state) && op.source2.input_ready(state)
            }
            Operation::LShift(op) | Operation::RShift(op) => op.source.input_ready(state),
        }
    }
}

fn parse_instruction(s: &str) -> Result<Operation, &str> {
    let operation = match s.split(" ").collect::<Vec<&str>>()[..] {
        [source, "->", target] => Operation::Assign(Op1 {
            source: source.parse::<ValueOrWire>().unwrap(),
            target: target.to_string(),
        }),
        ["NOT", source, "->", target] => Operation::Not(Op1 {
            source: source.parse::<ValueOrWire>().unwrap(),
            target: target.to_string(),
        }),
        [source1, "AND", source2, "->", target] => Operation::And(Op2 {
            source1: source1.parse::<ValueOrWire>().unwrap(),
            source2: source2.parse::<ValueOrWire>().unwrap(),
            target: target.to_string(),
        }),
        [source1, "OR", source2, "->", target] => Operation::Or(Op2 {
            source1: source1.parse::<ValueOrWire>().unwrap(),
            source2: source2.parse::<ValueOrWire>().unwrap(),
            target: target.to_string(),
        }),
        [source, "LSHIFT", amount, "->", target] => Operation::LShift(OpShift {
            source: source.parse::<ValueOrWire>().unwrap(),
            amount: amount.parse::<i32>().unwrap(),
            target: target.to_string(),
        }),
        [source, "RSHIFT", amount, "->", target] => Operation::RShift(OpShift {
            source: source.parse::<ValueOrWire>().unwrap(),
            amount: amount.parse::<i32>().unwrap(),
            target: target.to_string(),
        }),
        _ => return Err("Bad instruction"),
    };
    Ok(operation)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut instructions: Vec<Operation> = Vec::new();
    let mut state = State::new();
    for (i, line) in input.lines().enumerate() {
        instructions.push(parse_instruction(line).expect(&format!("Line {} has bad format", i)));
    }
    while let Some(instruction) = instructions.pop() {
        if instruction.is_executable(&state) {
            instruction.execute(&mut state).unwrap();
        } else {
            instructions.insert(0, instruction);
        }
    }
    println!("Value of wire \"a\": {}", state.get("a").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_execute() {
        let mut state = State::new();
        Operation::Assign(Op1 {
            source: ValueOrWire::Value(123),
            target: "x".to_string(),
        })
        .execute(&mut state)
        .unwrap();
        assert_eq!(state.get("x"), Some(&123));
        Operation::Assign(Op1 {
            source: ValueOrWire::Wire("x".to_string()),
            target: "y".to_string(),
        })
        .execute(&mut state)
        .unwrap();
        assert_eq!(state.get("y"), Some(&123));
    }

    #[test]
    fn test_operation_is_executable() {
        let mut state = State::new();
        let op1 = Operation::Assign(Op1 {
            source: ValueOrWire::Value(123),
            target: "x".to_string(),
        });
        assert_eq!(op1.is_executable(&state), true);
        let op2 = Operation::Assign(Op1 {
            source: ValueOrWire::Wire("x".to_string()),
            target: "y".to_string(),
        });
        assert_eq!(op2.is_executable(&state), false);
        op1.execute(&mut state).unwrap();
        assert_eq!(op2.is_executable(&state), true);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction(&"123 -> x").unwrap(),
            Operation::Assign(Op1 {
                source: ValueOrWire::Value(123),
                target: "x".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"x AND y -> d").unwrap(),
            Operation::And(Op2 {
                source1: ValueOrWire::Wire("x".to_string()),
                source2: ValueOrWire::Wire("y".to_string()),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"42 AND y -> d").unwrap(),
            Operation::And(Op2 {
                source1: ValueOrWire::Value(42),
                source2: ValueOrWire::Wire("y".to_string()),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"x AND 42 -> d").unwrap(),
            Operation::And(Op2 {
                source1: ValueOrWire::Wire("x".to_string()),
                source2: ValueOrWire::Value(42),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"x OR y -> d").unwrap(),
            Operation::Or(Op2 {
                source1: ValueOrWire::Wire("x".to_string()),
                source2: ValueOrWire::Wire("y".to_string()),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"42 OR y -> d").unwrap(),
            Operation::Or(Op2 {
                source1: ValueOrWire::Value(42),
                source2: ValueOrWire::Wire("y".to_string()),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"x OR 42 -> d").unwrap(),
            Operation::Or(Op2 {
                source1: ValueOrWire::Wire("x".to_string()),
                source2: ValueOrWire::Value(42),
                target: "d".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"x LSHIFT 2 -> f").unwrap(),
            Operation::LShift(OpShift {
                source: ValueOrWire::Wire("x".to_string()),
                amount: 2,
                target: "f".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"y RSHIFT 2 -> g").unwrap(),
            Operation::RShift(OpShift {
                source: ValueOrWire::Wire("y".to_string()),
                amount: 2,
                target: "g".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"NOT x -> h").unwrap(),
            Operation::Not(Op1 {
                source: ValueOrWire::Wire("x".to_string()),
                target: "h".to_string(),
            })
        );
        assert_eq!(
            parse_instruction(&"NOT 42 -> i").unwrap(),
            Operation::Not(Op1 {
                source: ValueOrWire::Value(42),
                target: "i".to_string(),
            })
        );
        assert!(parse_instruction(&"").is_err());
        assert!(parse_instruction(&"-> x").is_err());
        assert!(parse_instruction(&"123 ->").is_err());
        assert!(parse_instruction(&"123 -> 123 123").is_err());
    }

    #[test]
    fn test_value_or_wire_parse() {
        assert_eq!("123".parse::<ValueOrWire>(), Ok(ValueOrWire::Value(123)));
        assert_eq!(
            "a".parse::<ValueOrWire>(),
            Ok(ValueOrWire::Wire("a".to_string()))
        );
    }
}
