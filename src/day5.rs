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
    LoadOutput(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, i32),
    Equals(Parameter, Parameter, i32),
    Halt,
}

struct VM {
    mem: Mem,
    ip: usize,
    input: i32,
    output: i32,
}

impl VM {
    fn from_input(input: &str) -> Self {
        let opcodes = input
            .split(',')
            .map(|c: &str| c.trim_end_matches('\n').parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        VM {
            mem: Mem(opcodes),
            ip: 0,
            input: 1,
            output: 0,
        }
    }

    fn set_input(&mut self, input: i32) {
        self.input = input;
    }

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
            4 => Instruction::LoadOutput(Parameter(pm1.into(), self.fetch())),
            5 => Instruction::JumpIfTrue(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
            ),
            6 => Instruction::JumpIfFalse(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
            ),
            7 => Instruction::LessThan(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
                self.fetch(),
            ),
            8 => Instruction::Equals(
                Parameter(pm1.into(), self.fetch()),
                Parameter(pm2.into(), self.fetch()),
                self.fetch(),
            ),
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
                        self.output = self.mem.read(p1);
                        println!("load output {}", self.output);
                    }
                    Instruction::JumpIfTrue(p1, p2) => {
                        let cond = self.mem.read(p1);
                        if cond != 0 {
                            self.ip = self.mem.read(p2) as usize;
                        }
                    }
                    Instruction::JumpIfFalse(p1, p2) => {
                        let cond = self.mem.read(p1);
                        if cond == 0 {
                            self.ip = self.mem.read(p2) as usize;
                        }
                    }
                    Instruction::LessThan(p1, p2, p3) => {
                        let p1 = self.mem.read(p1);
                        let p2 = self.mem.read(p2);
                        if p1 < p2 {
                            self.mem.write(p3, 1);
                        } else {
                            self.mem.write(p3, 0);
                        }
                    }
                    Instruction::Equals(p1, p2, p3) => {
                        let p1 = self.mem.read(p1);
                        let p2 = self.mem.read(p2);
                        if p1 == p2 {
                            self.mem.write(p3, 1);
                        } else {
                            self.mem.write(p3, 0);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut vm = VM::from_input(include_str!("day5.input"));
    vm.set_input(5);
    vm.execute();
    println!("output: {}", vm.output);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_equalto_position_mode() {
        let mut vm = VM::from_input("3,9,8,9,10,9,4,9,99,-1,8");
        vm.set_input(8);
        vm.execute();
        assert_eq!(vm.output, 1);
    }

    #[test]
    fn test_equalto_immediate_mode() {
        let mut vm = VM::from_input("3,3,1108,-1,8,3,4,3,99");
        vm.set_input(8);
        vm.execute();
        assert_eq!(vm.output, 1);
    }

    #[test]
    fn test_lessthan_position_mode() {
        let mut vm = VM::from_input("3,9,7,9,10,9,4,9,99,-1,8");
        vm.set_input(8);
        vm.execute();
        assert_eq!(vm.output, 0);
    }

    #[test]
    fn test_lessthan_immediate_mode() {
        let mut vm = VM::from_input("3,3,1107,-1,8,3,4,3,99");
        vm.set_input(8);
        vm.execute();
        assert_eq!(vm.output, 0);
    }

    #[test]
    fn test_sample() {
        let mut vm = VM::from_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        vm.set_input(7);
        vm.execute();
        assert_eq!(vm.output, 999);
    }
}
