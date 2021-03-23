use crate::instructions::Opcode;
use log::{error, info};

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub program_counter: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub program_set_counter: i32,
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

    pub fn get_register_usage(&mut self) -> i16 {
        let mut used_reg_count: i16 = 0;
        let reg_copy = self.registers.clone();
        for i in reg_copy.iter() {
            if i.clone() != 0 as i32 {
                used_reg_count += 1;
            }
        }
        return used_reg_count;
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
        let result = ((self.program[self.program_counter] as u16) << 8)
            | self.program[self.program_counter + 1] as u16;
        self.program_counter += 2;
        return result;
    }

    // internal functions
    /// decodes opcode to Opcode object
    fn decode_opcode(&mut self) -> (Opcode, u8) {
        let original_op = self.program[self.program_counter];
        let opcode = Opcode::from(original_op);
        self.program_counter += 1;
        return (opcode, original_op);
    }

    /// executes VM call
    pub fn execute_vm_call(&mut self, call_name: i32, arg1: i32, arg2: i32) -> (bool, i32) {
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

            // exit call
            1 => {
                return (false, arg2);
            }
            _ => panic!(
                "Invalid VM call, {}  at program set: {} program counter: {} ",
                call_name, self.program_set_counter, self.program_counter
            ),
        }
        return (true, 0);
    }

    // execution functions
    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        loop {
            let (dont_kill, code) = self.execute_instruction();
            if !dont_kill {
                println!("process exited with code: {}", code);
                break;
            }
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> (bool, i32) {
        if self.program_counter >= self.program.len() {
            info!("program end reached");
            return (false, 0);
        }

        let (decoded_op, code) = self.decode_opcode();
        info!("got new instruction {}", code);

        match decoded_op {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                info!("Loading {} to R{}", number, register);
                // loads the number into the register
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return (false, 0);
            }
            Opcode::ADD => {
                let register1 = self.registers[self.registers[self.next_8_bits() as usize] as usize];
                let register2 = self.registers[self.registers[self.next_8_bits() as usize] as usize];
                let output_register = self.registers[self.next_8_bits() as usize];
                info!(
                    "adding {} + {} to R{}",
                    register1, register2, output_register
                );
                // loads the sum of register 1 & 2 into the
                self.registers[output_register as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.registers[self.next_8_bits() as usize] as usize];
                let register2 = self.registers[self.registers[self.next_8_bits() as usize] as usize];
                let output_register = self.registers[self.next_8_bits() as usize];
                print!(
                    "adding {} - {} to R{}",
                    register1, register2, output_register
                );
                // loads the subtraction of register 1 & 2 into the
                self.registers[output_register as usize] = register1 - register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.program_counter = target as usize;
                return (true, 0);
            }
            Opcode::RJMP => {
                let value = self.registers[self.next_8_bits() as usize];
                self.program_counter += value as usize;
                return (true, 0);
            }
            Opcode::VMCALL => {
                let call_name = self.registers[self.next_8_bits() as usize];
                let arg1 = self.registers[self.next_8_bits() as usize];
                let arg2 = self.registers[self.next_8_bits() as usize];
                info!("executing VMCALL {} {} {}", call_name, arg1, arg2);
                return self.execute_vm_call(call_name, arg1, arg2); // returns false if kill
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let output_register = self.registers[self.next_8_bits() as usize] as usize;
                if register1 == register2 {
                    self.registers[output_register as usize] = 1;
                } else {
                    self.registers[output_register as usize] = 0;
                }
            }
            Opcode::JEQ => {
                let source = self.registers[self.next_8_bits() as usize];
                let target = self.registers[self.next_8_bits() as usize];
                if source == 1 {
                    self.program_counter = target as usize;
                    return (true, 0);
                }
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let output_register = self.registers[self.next_8_bits() as usize] as usize;
                if register1 != register2 {
                    self.registers[output_register as usize] = 1;
                } else {
                    self.registers[output_register as usize] = 0;
                }
            }
            Opcode::JNEQ => {
                let source = self.registers[self.next_8_bits() as usize];
                let target = self.registers[self.next_8_bits() as usize];
                if source == 0 {
                    self.program_counter = target as usize;
                    return (true, 0);
                }
            }
            Opcode::SWP => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;
                let reg1v = self.registers[reg1];
                let reg2v = self.registers[reg2];

                self.registers[reg1] = reg2v;
                self.registers[reg2] = reg1v;
            }
            Opcode::AND => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let output_register = self.registers[self.next_8_bits() as usize] as usize;
                if register1 == 0 || register1 == 1 || register2 == 1 || register2 == 0 {
                    if register1 == 1 && register2 == 1 {
                        self.registers[output_register] = 1;
                    } else {
                        self.registers[output_register] = 0;
                    }
                } else {
                    error!(
                        "AND opcode arguments {} {} are not boolean",
                        register1, register2
                    )
                }
            }
            Opcode::OR => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let output_register = self.registers[self.next_8_bits() as usize] as usize;
                if register1 == 0 || register1 == 1 || register2 == 1 || register2 == 0 {
                    if (register1 == 1 && register2 == 1)
                        || (register1 == 1 && register2 == 0)
                        || (register1 == 0 && register2 == 1)
                    {
                        self.registers[output_register] = 1;
                    } else {
                        self.registers[output_register] = 0;
                    }
                } else {
                    error!(
                        "OR opcode arguments {} {} are not boolean",
                        register1, register2
                    )
                }
            }
            Opcode::NOT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let output_register = self.registers[self.next_8_bits() as usize] as usize;
                if register1 == 0 {
                    self.registers[output_register] = 1;
                } else if register1 == 1 {
                    self.registers[output_register] = 0;
                } else {
                    error!("NOT opcode arguments {} is not boolean", register1)
                }
            }
            _ => {
                println!(
                    "Unknown opcode at program set: {} program counter: {}",
                    self.program_set_counter, self.program_counter
                );
                return (false, 4);
            }
        }
        self.program_set_counter += 1;
        self.program_counter = (self.program_set_counter as usize) * (4 as usize);
        return (true, 0);
    }

    /// resets the register to original state
    pub fn clean_registers(&mut self) {
        for i in 0..(self.registers.len()) {
            self.registers[i] = 0;
        }
    }

    /// resets the program counter and set counter
    pub fn reset_program(&mut self) {
        self.program_counter = 0;
        self.program_set_counter = 0;
    }

    /// cleans the registers and resets the vm to original state
    pub fn reset_vm(&mut self) {
        self.reset_program();
        self.clean_registers();
        self.program = vec![0, 0, 0, 0];
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
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
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
        test_vm.registers[0] = 500;
        test_vm.registers[1] = 500;
        test_vm.registers[2] = 0;
        test_vm.registers[3] = 1;
        test_vm.registers[4] = 4;
        test_vm.program = vec![2, 2, 3, 4]; // load(opcode: 1) 500 into register 0
        test_vm.run();

        assert_eq!(test_vm.registers[4], 1000)
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 502;
        test_vm.registers[1] = 500;
        test_vm.registers[2] = 0;
        test_vm.registers[3] = 1;
        test_vm.registers[4] = 4;

        test_vm.program = vec![3, 2, 3, 4];
        test_vm.run();
        assert_eq!(test_vm.registers[4], 2)
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
        test_vm.program = vec![1, 0, 1, 255, 1, 1, 0, 0, 8, 1, 0, 0, 8, 1, 0, 0];
        test_vm.run();
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.registers[3] = 3;
        test_vm.program = vec![9, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1 as i32);
        test_vm.reset_program();
        test_vm.registers[1] = 25;
        test_vm.registers[3] = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 0 as i32);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1; // dest register
        test_vm.registers[2] = 7; // bool source register
        test_vm.program = vec![10, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 7);

        test_vm.registers[1] = 0;
        test_vm.reset_program();
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 4);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.registers[3] = 3;
        test_vm.program = vec![11, 0, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 0 as i32);
        test_vm.reset_program();
        test_vm.registers[1] = 25;
        test_vm.registers[3] = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1 as i32);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1; // dest register
        test_vm.registers[2] = 7; // bool source register
        test_vm.program = vec![12, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 4);

        test_vm.registers[1] = 0;
        test_vm.reset_program();
        test_vm.run_once();
        assert_eq!(test_vm.program_counter, 7);
    }

    #[test]
    fn test_swp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1; // dest register
        test_vm.registers[2] = 7; // bool source register
        test_vm.registers[3] = 1; // bool source register
        test_vm.registers[4] = 2; // bool source register

        test_vm.program = vec![13, 3, 4, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[1], 7);
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_and_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1;
        test_vm.registers[2] = 1;
        test_vm.registers[3] = 3;
        test_vm.program = vec![14, 1, 2, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);

        test_vm.registers[1] = 1;
        test_vm.registers[2] = 0;
        test_vm.registers[3] = 3;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3], 0);
    }

    #[test]
    fn test_or_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1;
        test_vm.registers[2] = 1;
        test_vm.registers[3] = 3;
        test_vm.program = vec![15, 1, 2, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);

        test_vm.registers[1] = 1;
        test_vm.registers[2] = 0;
        test_vm.registers[3] = 3;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);

        test_vm.registers[1] = 0;
        test_vm.registers[2] = 1;
        test_vm.registers[3] = 3;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);

        test_vm.registers[1] = 0;
        test_vm.registers[2] = 0;
        test_vm.registers[3] = 3;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3], 0);
    }

    #[test]
    fn test_not_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1;
        test_vm.registers[3] = 3;
        test_vm.program = vec![16, 1, 3, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 0);

        test_vm.registers[1] = 0;
        test_vm.registers[3] = 3;
        test_vm.program = vec![16, 1, 3, 0];
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);
    }
}
