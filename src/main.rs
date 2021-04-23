use crate::vm::VM;
use log::info;
use simplelog::*;
use std::fs::File;
use std::mem;
use clap::{App, load_yaml};
use home;
use std::fs;

pub mod instructions;
pub mod register;
pub mod stack;
pub mod vm;
mod debug;
mod label;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let location = matches.value_of("FILE").unwrap();
    let mut register_file_location = "";

    if let Some(x) = matches.value_of("reg") {
        register_file_location = x;
    }
    println!("{}",register_file_location);
    let info_log_filter = match matches.is_present("loginfo") {
        true => LevelFilter::Info,
        _ => LevelFilter::Off,
    };

    let log_location = match home::home_dir() {
        Some(path) => format!("{}{}",path.to_str().unwrap(),"/perlingvm/logs"),
        None => "".to_string(),
    };
    println!("{}{}",log_location,"/perling.info.log");

    fs::create_dir_all(&log_location).unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            info_log_filter,
            Config::default(),
            File::create(format!("{}{}",log_location,"/perling.info.log")).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            File::create("perling.info.log").unwrap(),
        ),
    ])
    .unwrap();

    use std::io::Read;
    let file = std::fs::File::open(location)
        .unwrap()
        .bytes()
        .map(|ch| ch.unwrap());

    let mut vm = VM::new();
    for i in file {
        vm.program.append(&mut vec![i]);
    }
    if register_file_location != "" {
        let mut buffer = String::new();
        std::fs::File::open(register_file_location).unwrap().read_to_string(&mut buffer).unwrap();
        println!("{}",buffer);
        register::register_from_string(&buffer, &mut vm.registers)
    }
    vm.run();
    info!("process used {} register(s)", vm.get_register_usage());
    info!("process was allocated {}B", mem::size_of_val(&vm.registers))
}
