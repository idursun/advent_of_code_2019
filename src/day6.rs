use std::collections::BTreeMap;

fn solve(input: &str) -> usize {
    let mut deps = BTreeMap::<&str, Vec<&str>>::new();
    let mut counts = BTreeMap::<&str, usize>::new();

    for line in input.lines() {
        let mut split = line.split(')');
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        let deps = deps.entry(first).or_insert(Vec::new());
        deps.push(second);
    }

    for (key, children) in &deps {
        let entry = *counts.entry(key).or_insert(0);
        for child in children {
            *counts.entry(child).or_insert(0) += entry + 1;
        }
        println!("{}: {:?}", key, children);
    }
    0
}

fn main() {
    let input = include_str!("day6.input");
    let count = solve(input);
    println!("count = {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_backtrack_dependencies() {
        let count = solve(
            r#"B)C
A)B"#,
        );

        assert_eq!(count, 3);
    }

    #[test]
    fn test_sample() {
        let count = solve(
            r#"D)E
J)K
D)I
E)F
COM)B
B)G
B)C
G)H
C)D
E)J
K)L"#,
        );

        assert_eq!(count, 42);
    }
}
