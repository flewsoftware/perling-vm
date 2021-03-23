use crate::vm::VM;
use log::info;
use simplelog::*;
use std::env;
use std::fs::File;
use std::mem;

pub mod instructions;
pub mod stack;
pub mod vm;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("perling.info.log").unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            File::create("perling.info.log").unwrap(),
        ),
    ])
    .unwrap();

    let args: Vec<String> = env::args().collect();

    use std::io::Read;
    let file = std::fs::File::open(args[1].to_string())
        .unwrap()
        .bytes()
        .map(|ch| ch.unwrap());
    let mut vm = VM::new();
    for i in file {
        vm.program.append(&mut vec![i]);
    }
    vm.run();
    info!("process used {} register(s)", vm.get_register_usage());
    info!("process was allocated {}B", mem::size_of_val(&vm.registers))
}
