fn main() {
    let input = include_str!("day1.input");
    let mut total = 0;
    for mass in input.lines() {
        let mut m = str::parse::<i32>(mass).unwrap();
        while m > 0 {
            m = i32::max(m / 3 - 2, 0);
            total += m;
        }
    }
    println!("{}", total);
}
