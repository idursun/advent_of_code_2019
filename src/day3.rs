#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Instruction(Direction, i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point(i32, i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("invalid direction"),
        }
    }
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }

    fn is_horizontal(&self) -> bool {
        self.from.0 != self.to.0
    }

    fn get_x(&self) -> (i32, i32) {
        let min = std::cmp::min(self.from.0, self.to.0);
        let max = std::cmp::max(self.from.0, self.to.0);
        (min, max)
    }

    fn get_y(&self) -> (i32, i32) {
        let min = std::cmp::min(self.from.1, self.to.1);
        let max = std::cmp::max(self.from.1, self.to.1);
        (min, max)
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let direction: Direction = input.chars().next().unwrap().into();
        let rest = input.get(1..).unwrap();
        Instruction(direction, rest.parse::<i32>().unwrap())
    }
}

fn get_line_segments(start: Point, instructions: Vec<Instruction>) -> Vec<Line> {
    let mut result = Vec::new();
    let mut current = start;
    for inst in instructions {
        let new_point = match inst {
            Instruction(Direction::Right, ref amount) => Point(current.0 + amount, current.1),
            Instruction(Direction::Left, ref amount) => Point(current.0 - amount, current.1),
            Instruction(Direction::Up, ref amount) => Point(current.0, current.1 + amount),
            Instruction(Direction::Down, ref amount) => Point(current.0, current.1 - amount),
        };
        result.push(Line {
            from: current.clone(),
            to: new_point,
        });
        current = new_point;
    }
    result
}

fn main() {
    let input = include_str!("day3.input").lines().collect::<Vec<_>>();

    let line0 = input[0]
        .split(',')
        .map(Instruction::from)
        .collect::<Vec<_>>();

    let line1 = input[1]
        .split(',')
        .map(Instruction::from)
        .collect::<Vec<_>>();

    let segments0 = get_line_segments(Point(0, 0), line0);
    let segments1 = get_line_segments(Point(0, 0), line1);
    let mut min_dist = i32::max_value();
    for sega in &segments0 {
        for segb in &segments1 {
            if sega.is_horizontal() && segb.is_vertical() {
                let (min_ax, max_ax) = sega.get_x();
                let (min_by, max_by) = segb.get_y();

                if min_ax < segb.from.0
                    && segb.from.0 < max_ax
                    && min_by < sega.from.1
                    && sega.from.1 < max_by
                {
                    let dist = segb.from.0.abs() + sega.from.1.abs();
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
            }

            if sega.is_vertical() && segb.is_horizontal() {
                let (min_ay, max_ay) = sega.get_y();
                let (min_bx, max_bx) = segb.get_x();

                if min_ay < segb.from.1
                    && segb.from.1 < max_ay
                    && min_bx < sega.from.0
                    && sega.from.0 < max_bx
                {
                    let dist = segb.from.1.abs() + sega.from.0.abs();
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
            }
        }
    }
    println!("{}", min_dist);
}
