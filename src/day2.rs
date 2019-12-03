fn simulate(mut opcodes: Vec<i32>, noun: i32, verb: i32) -> i32 {
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut ip = 0;
    while opcodes[ip] != 99 {
        let opcode = opcodes[ip];
        match opcode {
            1 => {
                let ptr1 = opcodes[ip + 1] as usize;
                let ptr2 = opcodes[ip + 2] as usize;
                let result = opcodes[ptr1] + opcodes[ptr2];
                let dst = opcodes[ip + 3];
                opcodes[dst as usize] = result;
                ip += 4;
            }
            2 => {
                let ptr1 = opcodes[ip + 1] as usize;
                let ptr2 = opcodes[ip + 2] as usize;
                let result = opcodes[ptr1] * opcodes[ptr2];
                let dst = opcodes[ip + 3];
                opcodes[dst as usize] = result;
                ip += 4;
            }
            99 => {
                break;
            }
            _ => panic!("unknown operand"),
        }
    }
    opcodes[0]
}

fn main() {
    let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,10,23,27,2,27,13,31,1,10,31,35,1,35,9,39,2,39,13,43,1,43,5,47,1,47,6,51,2,6,51,55,1,5,55,59,2,9,59,63,2,6,63,67,1,13,67,71,1,9,71,75,2,13,75,79,1,79,10,83,2,83,9,87,1,5,87,91,2,91,6,95,2,13,95,99,1,99,5,103,1,103,2,107,1,107,10,0,99,2,0,14,0";
    let opcodes = input
        .split(',')
        .map(|c: &str| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    const TARGET: i32 = 19690720;
    let mut verb = 0;
    let mut noun = 0;
    'outer: while verb < 100 {
        while noun < 100 {
            noun += 1;
            if simulate(opcodes.clone(), noun, verb) == TARGET {
                break 'outer;
            }
        }
        noun = 0;
        verb += 1;
    }
    println!("found: {}", noun * 100 + verb);
}
