use crate::vm::VM;
use std::env;

pub mod vm;
pub mod instructions;

fn main() {
    let args: Vec<String> = env::args().collect();

    use std::io::Read;
    let file = std::fs::File::open(args[1].to_string()).unwrap().bytes().map(|ch| ch.unwrap());
    let mut vm = VM::new();
    for i in file {
        vm.program.append(&mut vec!(i));
    }
    vm.run();
}