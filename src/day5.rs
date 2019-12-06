#[derive(Debug)]
enum ParameterMode {
    Address,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(input: i32) -> Self {
        if input == 0 {
            Self::Address
        } else {
            Self::Immediate
        }
    }
}

#[derive(Debug)]
struct Parameter(ParameterMode, i32);

#[derive(Debug)]
struct Mem(Vec<i32>);

impl Mem {
    fn read(&self, param: Parameter) -> i32 {
        match param {
            Parameter(ParameterMode::Address, p) => {
                let v = self.0[p as usize];
                println!("\tread [{}]={}", p, v);
                v
            }
            Parameter(ParameterMode::Immediate, p) => p,
        }
    }

    fn write(&mut self, pos: i32, value: i32) {
        self.0[pos as usize] = value;
        println!("\twrite [{}]={}", pos, value);
    }
}

#[derive(Debug)]
enum Instruction {
    Addition(Parameter, Parameter, i32),
    Multiply(Parameter, Parameter, i32),
    LoadInput(i32),
    LoadOutput(i32),
    Halt,
}

struct VM {
    mem: Mem,
    ip: usize,
    input: i32,
    output: i32,
}

impl VM {
    fn fetch(&mut self) -> i32 {
        let ret = self.mem.0[self.ip];
        self.ip += 1;
        ret
    }

    fn decode(&mut self) -> Instruction {
        let opcode = self.fetch();
        let lowdigits = opcode % 100;
        let pm1 = (opcode / 100) % 10;
        let pm2 = (opcode / 1000) % 10;
        match lowdigits {
            1 => Instruction::Addition(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
                self.fetch(),
            ),
            2 => Instruction::Multiply(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
                self.fetch(),
            ),
            3 => Instruction::LoadInput(self.fetch()),
            4 => Instruction::LoadOutput(self.fetch()),
            99 => Instruction::Halt,
            _ => panic!("unexpected opcode {}", lowdigits),
        }
    }

    fn execute(&mut self) {
        'outer: while self.ip < self.mem.0.len() {
            loop {
                let cip = self.ip;
                let current = self.decode();
                println!("{}: {:?}", cip, current);
                match current {
                    Instruction::Halt => break 'outer,
                    Instruction::Addition(p1, p2, p3) => {
                        let v1 = self.mem.read(p1);
                        let v2 = self.mem.read(p2);
                        self.mem.write(p3, v1 + v2);
                    }
                    Instruction::Multiply(p1, p2, p3) => {
                        let v1 = self.mem.read(p1);
                        let v2 = self.mem.read(p2);
                        self.mem.write(p3, v1 * v2);
                    }
                    Instruction::LoadInput(p1) => {
                        self.mem.write(p1, self.input);
                    }
                    Instruction::LoadOutput(p1) => {
                        self.output = self.mem.read(Parameter(ParameterMode::Address, p1));
                        println!("load output {}", self.output);
                    }
                }
            }
        }
    }
}

fn main() {
    //let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,10,23,27,2,27,13,31,1,10,31,35,1,35,9,39,2,39,13,43,1,43,5,47,1,47,6,51,2,6,51,55,1,5,55,59,2,9,59,63,2,6,63,67,1,13,67,71,1,9,71,75,2,13,75,79,1,79,10,83,2,83,9,87,1,5,87,91,2,91,6,95,2,13,95,99,1,99,5,103,1,103,2,107,1,107,10,0,99,2,0,14,0";
    let input = include_str!("day5.input");
    let opcodes = input
        .split(',')
        .map(|c: &str| c.trim_end_matches('\n').parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut vm = VM {
        mem: Mem(opcodes),
        ip: 0,
        input: 1,
        output: 0,
    };

    vm.execute();
    println!("output: {}", vm.output);
}
