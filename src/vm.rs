use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            println!("Executing at pc: {}", self.pc);
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as i32;
                println!("LOAD {} into register {}", number, register);
                self.registers[register] = number;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                println!("ADD result: {} + {} = {}", register1, register2, register1 + register2);
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                println!("MUL result: {} * {} = {}", register1, register2, register1 * register2);
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register2 != 0 {
                    self.registers[dest] = register1 / register2;
                    self.remainder = (register1 % register2) as u32;
                    println!("DIV result: {} / {} = {}, remainder = {}", register1, register2, self.registers[dest], self.remainder);
                } else {
                    println!("Error: Division by zero");
                    self.registers[dest] = 0;
                    self.remainder = register1 as u32;
                }
            },
            Opcode::JMP => {
                let address = self.next_16_bits() as usize;
                println!("Jumping to absolute address {}", address);
                self.pc = address;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let destination = self.next_8_bits() as usize;
                self.registers[destination] = register1 - register2;
                println!("SUB result: {} - {} = {}", register1, register2, self.registers[destination]);
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::IGL => {
                println!("Unknown opcode encountered at {}", self.pc - 1);
                return true;
            },
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 3;
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 3);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_opcode_div_by_zero() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 0;
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        assert_eq!(test_vm.remainder, 10);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.program = vec![5, 0, 10];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 10);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 3;
        test_vm.program = vec![6, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 7);
    }
}