fn increasing(input: &str) -> bool {
    let mut cs = input.chars();
    let mut p = cs.next().unwrap();
    while let Some(n) = cs.next() {
        if n < p {
            return false;
        }
        p = n;
    }
    return true;
}

fn repeating(input: &str) -> bool {
    let mut cs = input.chars();
    let mut p = cs.next().unwrap();
    let mut same_count = 1;
    while let Some(n) = cs.next() {
        if n == p {
            same_count += 1;
        } else if same_count == 2 {
            return true;
        } else {
            same_count = 1;
        }
        p = n;
    }
    return same_count == 2;
}

fn main() {
    const LOW: i32 = 265275;
    const HIGH: i32 = 781584;

    let mut found = 0;
    for current in LOW..=HIGH {
        let input = current.to_string();
        let input = &input[..];
        if increasing(input) && repeating(input) {
            found += 1;
        }
    }
    println!("{}", found);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeating() {
        assert_eq!(repeating("112222"), true);
    }

    #[test]
    fn test_repeating2() {
        assert_eq!(repeating("222233"), true);
    }

    #[test]
    fn test_repeating3() {
        assert_eq!(repeating("222234"), false);
    }
}
