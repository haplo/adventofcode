use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn deliver_presents(plan: &str) -> HashMap<Point, u32> {
    let mut current_pos = Point { x: 0, y: 0 };
    let mut visits = HashMap::new();
    visits.insert(current_pos.clone(), 1);
    for char in plan.chars() {
        match char {
            '<' => current_pos.x -= 1,
            '>' => current_pos.x += 1,
            'v' => current_pos.y -= 1,
            '^' => current_pos.y += 1,
            _ => (),
        }
        visits.insert(
            current_pos.clone(),
            match visits.get(&current_pos) {
                Some(n) => n + 1,
                None => 1,
            },
        );
    }
    visits
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let visits = deliver_presents(&input);
    println!("{} houses with at least one present", visits.keys().count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(deliver_presents(">").keys().count(), 2);
        assert_eq!(deliver_presents("^>v<").keys().count(), 4);
        assert_eq!(deliver_presents("^v^v^v^v^v").keys().count(), 2);
    }
}
