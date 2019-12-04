fn main() {
    const LOW: i32 = 265275;
    const HIGH: i32 = 781584;

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
        while let Some(n) = cs.next() {
            if n == p {
                return true;
            }
            p = n;
        }
        return false;
    }

    let mut found = 0;
    for current in LOW..=HIGH {
        let input = current.to_string();
        let input = &input[..];
        if increasing(input) && repeating(input) {
            println!("{}", input);
            found += 1;
        }
    }
    println!("{}", found);
}
