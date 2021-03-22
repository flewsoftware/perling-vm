use crate::instructions::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub program_counter: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub program_set_counter: i32
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            program_counter: 0,
            program_set_counter: 0,
            remainder: 0,
        }
    }

    // byte access
    /// returns the next 8 bits and increments the program counter
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.program_counter];
        self.program_counter += 1;
        return result;
    }

    /// returns the next 16 bits and increments the program counter
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.program_counter] as u16) << 8) | self.program[self.program_counter + 1] as u16;
        self.program_counter += 2;
        return result;
    }


    // internal functions
    /// decodes opcode to Opcode object
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.program_counter]);
        self.program_counter += 1;
        return opcode;
    }

    /// executes VM call
    pub fn execute_vm_call(&mut self, call_name: i32, arg1: i32, arg2: i32) {
        match call_name {

            // print call
            0 => {
                // print mode
                if arg1 == 0 {
                    print!("{}", arg2)
                } else {
                    println!("{}", arg2)
                }
            }

            _ => {
                panic!("Invalid VM call, {}  at program set: {} program counter: {} ", call_name, self.program_set_counter, self.program_counter)
            }
        }
    }

    // execution functions
    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }


    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                // loads the number into the register
                self.registers[register] = number as i32;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                // loads the sum of register 1 & 2 into the
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                // loads the subtraction of register 1 & 2 into the
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.program_counter = target as usize;
                return false;
            },
            Opcode::RJMP => {
                let value = self.registers[self.next_8_bits() as usize];
                self.program_counter += value as usize;
                return false;
            },
            Opcode::VMCALL => {
                let call_name = self.registers[self.next_8_bits() as usize];
                let arg1 = self.registers[self.next_8_bits() as usize];
                let arg2 = self.registers[self.next_8_bits() as usize];
                self.execute_vm_call(call_name, arg1, arg2);
            },
            _ => {
                println!("Unknown opcode at program set: {} program counter: {}", self.program_set_counter, self.program_counter);
                return true;
            },
        }
        self.program_set_counter += 1;
        self.program_counter = (self.program_set_counter as usize) * (4 as usize);
        false
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        // tests if vm will halt when no instructions
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244]; // this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244,  1,1,1,244, 2,0,1,2]; // load(opcode: 1) 500 into register 0
        test_vm.run();

        assert_eq!(test_vm.registers[2], 1000)
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244, 1, 1, 1, 245, 3, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1)
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![5, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![6, 0, 0, 0, 0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 4);
    }

    #[test]
    fn test_vmcall_print_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![1,0,1,255, 1,1,0,0, 8,1,0,0, 8,1,0,0];
        test_vm.run();
    }
}