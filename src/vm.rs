use crate::instructions::Opcode;
use crate::register::REGISTER;
use crate::stack::STACK;
use log::{error, info};
use crate::debug::DebugEngine;
use crate::label::LABEL;

#[derive(Debug)]
pub struct VM {
    pub registers: [REGISTER; 32],
    pub program_counter: usize,     // current byte
    pub program: Vec<u8>,           // program instructions
    pub remainder: i32,             // remainder of div opcode
    pub program_set_counter: i32,   // current line
    pub stack: STACK,               // stack data
    pub labels: Vec<LABEL>          // label data
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [REGISTER {
                content: 0,
                locked: false,
            }; 32],
            program: vec![],
            program_counter: 0,
            program_set_counter: 0,
            remainder: 0,
            stack: STACK { content: vec![0] },
            labels: vec![LABEL{ id: 0, location: 0 }],
        }
    }

    pub fn get_register_usage(&mut self) -> i16 {
        let mut used_reg_count: i16 = 0;
        let reg_copy = self.registers.clone();
        for i in reg_copy.iter() {
            if i.clone().content != 0 as i32 {
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
                self.registers[register].do_set(number as i32);
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return (false, 0);
            }
            Opcode::ADD => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content;
                // loads the sum of register 1 & 2 into the
                self.registers[output_register as usize].do_set(register1 + register2);
            }
            Opcode::SUB => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content;
                // loads the subtraction of register 1 & 2 into the
                self.registers[output_register as usize].do_set(register1 - register2);
            }
            Opcode::DIV => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                self.registers[self.next_8_bits() as usize].do_set(register1 / register2);
                self.remainder = (register1 % register2) as i32;
            }
            Opcode::JMP => {
                let current_pos = self.program_counter;
                let target = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                self.program_set_counter = target;
                self.program_counter = 0;

                info!("jumped from {} to {}", current_pos, target);
                return (true, 0);
            }
            Opcode::RJMP => {
                let current_pos = self.program_counter;
                let value = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                self.program_set_counter += value;
                self.program_counter = 0;
                info!("jumped from {} to {}", current_pos, self.program_counter);
                return (true, 0);
            }
            Opcode::VMCALL => {
                let call_name = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let arg1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let arg2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                info!("executing VMCALL {} {} {}", call_name, arg1, arg2);
                return self.execute_vm_call(call_name, arg1, arg2); // returns false if kill
            }
            Opcode::EQ => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                if register1 == register2 {
                    self.registers[output_register as usize].do_set(1);
                } else {
                    self.registers[output_register as usize].do_set(0);
                }
            }
            Opcode::JEQ => {
                let current_pos = self.program_counter;
                let source = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let target = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                if source == 1 {
                    self.program_counter = 0 as usize;
                    self.program_set_counter = target;
                    return (true, 0);
                }
                info!("jumped from {} to {}", current_pos, target);
            }
            Opcode::NEQ => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                if register1 != register2 {
                    self.registers[output_register as usize].do_set(1);
                } else {
                    self.registers[output_register as usize].do_set(0);
                }
            }
            Opcode::JNEQ => {
                let current_pos = self.program_counter;
                let source = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let target = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;

                if source == 0 {
                    self.program_set_counter = target;
                    self.program_counter = 0 as usize;
                    info!("jumped from {} to {}", current_pos, target);
                    return (true, 0);
                }
            }
            Opcode::SWP => {
                let reg1 = self.registers[self.next_8_bits() as usize].content as usize;
                let reg2 = self.registers[self.next_8_bits() as usize].content as usize;
                let reg1v = self.registers[reg1].content;
                let reg2v = self.registers[reg2].content;

                self.registers[reg1].do_set(reg2v);
                self.registers[reg2].do_set(reg1v);

                info!("swaped R{} with R{}", reg1, reg2)
            }
            Opcode::AND => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                if register1 == 0 || register1 == 1 || register2 == 1 || register2 == 0 {
                    if register1 == 1 && register2 == 1 {
                        self.registers[output_register].do_set(1);
                    } else {
                        self.registers[output_register].do_set(0);
                    }
                } else {
                    error!(
                        "AND opcode arguments {} {} are not boolean",
                        register1, register2
                    )
                }
            }
            Opcode::OR => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let register2 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                if register1 == 0 || register1 == 1 || register2 == 1 || register2 == 0 {
                    if (register1 == 1 && register2 == 1)
                        || (register1 == 1 && register2 == 0)
                        || (register1 == 0 && register2 == 1)
                    {
                        self.registers[output_register].do_set(1);
                    } else {
                        self.registers[output_register].do_set(0);
                    }
                } else {
                    error!(
                        "OR opcode arguments {} {} are not boolean",
                        register1, register2
                    )
                }
            }
            Opcode::NOT => {
                let register1 = self.registers
                    [self.registers[self.next_8_bits() as usize].content as usize]
                    .content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                if register1 == 0 {
                    self.registers[output_register].do_set(1);
                } else if register1 == 1 {
                    self.registers[output_register].do_set(0);
                } else {
                    error!("NOT opcode arguments {} is not boolean", register1)
                }
            }
            Opcode::GET => {
                let hidden_register_id = self.registers[self.next_8_bits() as usize].content;
                let output_register = self.registers[self.next_8_bits() as usize].content as usize;
                match hidden_register_id {
                    // remainder register
                    0 => {
                        self.registers[output_register].do_set(self.remainder);
                        self.remainder = 0;
                    }
                    _ => {
                        self.registers[output_register].do_set(0);
                    }
                }
                info!(
                    "Moved hidden R{} content to R{}",
                    hidden_register_id, output_register,
                )
            }
            Opcode::LOCKR => {
                let register_to_toggle_lock =
                    self.registers[self.next_8_bits() as usize].content as usize;
                self.registers[register_to_toggle_lock].toggle_lock();
                info!(
                    "R{} is now locked:{}",
                    register_to_toggle_lock, self.registers[register_to_toggle_lock].locked,
                )
            }
            Opcode::PUSHRTS => {
                let target_register = self.registers[self.next_8_bits() as usize].content as usize;
                self.stack.add_register(self.registers[target_register]);
                self.registers[target_register].content = 0;
            }
            Opcode::POPRFS => {
                let target_register = self.registers[self.next_8_bits() as usize].content as usize;
                self.registers[target_register].content = self.stack.content.pop().unwrap();
            }
            Opcode::BREAK => {
                println!("hit BREAK on line:{}", self.program_set_counter);
                let mut d = DebugEngine{};
                d.wait_for_commands(self, std::io::stdin());
            }
            Opcode::LABEL => {
                let label_id = self.next_8_bits() as i32;
                self.labels.append(&mut vec![LABEL{ id: label_id, location: self.program_set_counter }]);
            }
            Opcode::GOTO => {
                let label_id = self.next_8_bits() as i32;
                let mut found = false;
                for label in self.labels.iter() {
                    if label.id == label_id {
                        self.program_set_counter = label.location;
                        self.program_counter = 0;
                        found = true;
                        break
                    }
                }
                if !found {
                    error!("GOTO label {} not found", label_id);
                }
                return (true, 0)
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
            self.registers[i].content = 0;
        }
    }

    /// resets the program counter and set counter
    pub fn reset_program(&mut self) {
        self.program_counter = 0;
        self.program_set_counter = 0;
    }

    #[deprecated (note="please replace VM instance with a new one instead of reset_vm")]
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
        assert_eq!(test_vm.registers[0].content, 0)
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
        assert_eq!(test_vm.registers[0].content, 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 500;
        test_vm.registers[1].content = 500;
        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 1;
        test_vm.registers[4].content = 4;
        test_vm.program = vec![2, 2, 3, 4]; // load(opcode: 1) 500 into register 0
        test_vm.run();

        assert_eq!(test_vm.registers[4].content, 1000)
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 7;
        test_vm.registers[1].content = 3;

        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 1;

        test_vm.registers[4].content = 4;
        test_vm.program = vec![4, 2, 3, 4]; // load(opcode: 1) 500 into register 0
        test_vm.run();

        assert_eq!(test_vm.registers[4].content, 2);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 502;
        test_vm.registers[1].content = 500;
        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 1;
        test_vm.registers[4].content = 4;

        test_vm.program = vec![3, 2, 3, 4];
        test_vm.run();
        assert_eq!(test_vm.registers[4].content, 2)
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 1;
        test_vm.registers[1].content = 0;
        test_vm.program = vec![5, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 1);
    }

    #[test]
    fn test_rjmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 2;
        test_vm.registers[1].content = 0;
        test_vm.program = vec![6, 1, 0, 0, 0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 2);
    }

    #[test]
    fn test_vmcall_print_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 255; // print
        test_vm.registers[1].content = 0; // print ref

        test_vm.registers[2].content = 0; // arg1
        test_vm.registers[3].content = 2; // arg1 ref

        test_vm.registers[4].content = 1; // arg2
        test_vm.registers[5].content = 4; // arg2 ref

        test_vm.program = vec![8, 3, 5, 1];
        test_vm.run();
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 10;
        test_vm.registers[1].content = 10;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 0;
        test_vm.registers[5].content = 1;
        test_vm.program = vec![9, 4, 5, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1 as i32);

        test_vm.reset_program();
        test_vm.registers[1].content = 25;
        test_vm.registers[3].content = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 0 as i32);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1; // dest register
        test_vm.registers[2].content = 7; // bool source register

        test_vm.registers[3].content = 1;
        test_vm.registers[4].content = 2;
        test_vm.program = vec![10, 3, 4, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 7);

        test_vm.registers[1].content = 0;
        test_vm.reset_program();
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 1);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0].content = 10;
        test_vm.registers[1].content = 10;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 0;
        test_vm.registers[5].content = 1;
        test_vm.program = vec![11, 4, 5, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 0 as i32);
        test_vm.reset_program();
        test_vm.registers[1].content = 25;
        test_vm.registers[3].content = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1 as i32);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1; // dest register
        test_vm.registers[2].content = 7; // bool source register

        test_vm.registers[3].content = 1;
        test_vm.registers[4].content = 2;
        test_vm.program = vec![12, 3, 4, 0];
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 1);

        test_vm.registers[1].content = 0;
        test_vm.reset_program();
        test_vm.run_once();
        assert_eq!(test_vm.program_set_counter, 7);
    }

    #[test]
    fn test_swp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1; // dest register
        test_vm.registers[2].content = 7; // bool source register

        test_vm.registers[3].content = 1; // dest register ref
        test_vm.registers[4].content = 2; // bool source register ref

        test_vm.program = vec![13, 3, 4, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[1].content, 7);
        assert_eq!(test_vm.registers[2].content, 1);
    }

    #[test]
    fn test_and_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1;
        test_vm.registers[2].content = 1;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.program = vec![14, 4, 5, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1);

        test_vm.registers[1].content = 1;
        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 0);
    }

    #[test]
    fn test_or_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1;
        test_vm.registers[2].content = 1;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.program = vec![15, 4, 5, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1);

        test_vm.registers[1].content = 1;
        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1);

        test_vm.registers[1].content = 0;
        test_vm.registers[2].content = 1;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1);

        test_vm.registers[1].content = 0;
        test_vm.registers[2].content = 0;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.registers[5].content = 2;
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 0);
    }

    #[test]
    fn test_not_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 1;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.program = vec![16, 4, 3, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 0);

        test_vm.registers[1].content = 0;
        test_vm.registers[3].content = 3;

        test_vm.registers[4].content = 1;
        test_vm.program = vec![16, 4, 3, 0];
        test_vm.reset_program();
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 1);
    }

    #[test]
    fn test_get_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 0;
        test_vm.registers[3].content = 3;
        test_vm.remainder = 2;
        test_vm.program = vec![17, 1, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3].content, 2);
    }

    #[test]
    fn test_prts_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1].content = 2;
        test_vm.registers[3].content = 1;
        test_vm.remainder = 2;
        test_vm.program = vec![19, 3, 0];
        test_vm.run();
        assert_eq!(test_vm.stack.content.pop(), Some(2));
        assert_eq!(test_vm.registers[1].content, 0)
    }
    #[test]
    fn test_poprfs_opcode() {
        let mut test_vm = VM::new();
        test_vm.stack.content.push(10);
        test_vm.registers[1].content = 0;
        test_vm.registers[3].content = 1;
        test_vm.remainder = 2;
        test_vm.program = vec![20, 3, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[1].content, 10)
    }
    #[test]
    fn test_label_opcode() {
        let mut test_vm = VM::new();
        test_vm.stack.content.push(10);
        test_vm.remainder = 2;
        test_vm.program = vec![1, 1, 1, 1, 22, 3, 0, 0,  1, 1, 1, 1,  1, 1, 1, 1];
        test_vm.run();
        assert_eq!(test_vm.labels[1].id, 3);
        assert_eq!(test_vm.labels[1].location, 1);
    }
    #[test]
    fn test_goto_opcode() {
        let mut test_vm = VM::new();
        test_vm.stack.content.push(10);
        test_vm.remainder = 2;
        test_vm.program = vec![1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  23, 3, 0, 0];
        test_vm.labels.append(&mut vec![LABEL { id: 3, location: 2 }]);
        for _ in 0..4 {
            test_vm.run_once();
        }

        assert_eq!(test_vm.program_counter, 0);
        assert_eq!(test_vm.program_set_counter, 2);
    }
}
