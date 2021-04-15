use std::io::{BufRead};
use crate::VM;
pub struct DebugEngine {

}

impl DebugEngine {

   pub fn wait_for_commands(&mut self, vm: &VM, stdin: std::io::Stdin) {
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            let stop = self.handle_command(l, vm);
            if stop {
                break;
            }
        }
    }

   pub fn handle_command(&mut self, command: String, vm: &VM) -> bool {
        let command_data: Vec<&str> = command.split(" ").collect();
        match command_data[0] {
            "print_registers" => {
                for i in 0..vm.registers.len() {
                    println!("{}:\t{}\tlocked:{}", i, vm.registers[i].content, vm.registers[i].locked)
                }
                println!("h0:\t{}\tremainder register", vm.remainder)
            },
            "print_registers_non_zero" => {
                for i in 0..vm.registers.len() {
                    if vm.registers[i].content == 0 {
                        continue;
                    }
                    println!("{}:\t{}\tlocked:{}", i, vm.registers[i].content, vm.registers[i].locked)
                }
                println!("h0:\t{}\tremainder register", vm.remainder)
            }
            "continue" => {
                return true;
            }
            "help" => {
                println!(
                    "print_registers\tprints register contents\n\
                    print_register_non_zero\tprints register contents that are not 0\n\
                    continue\tcontinues program execution\n"
                )
            }
            _ => println!("cant find command")
        }
       return false;
    }
}