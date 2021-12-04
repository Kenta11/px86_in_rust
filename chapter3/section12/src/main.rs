mod emulator;
mod emulator_function;
mod instruction;

use crate::instruction::New;
use clap::{App, Arg};
use emulator::Emulator;
use instruction::InstructionFunctions;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    const MEMORY_SIZE: usize = 1_000_000;
    const PROGRAM_HEAD: usize = 0x7C00;
    const PROGRAM_SIZE: usize = 512;

    let matches = App::new("Pico x86 emulator")
        .version("1.0.0")
        .author("Kenta11")
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("filename")
                .value_name("FILE")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let path = matches.value_of("filename").unwrap();

    let mut emu = Emulator::new(MEMORY_SIZE, PROGRAM_HEAD as u32, PROGRAM_HEAD as u32);

    let functions = InstructionFunctions::new();

    let f = File::open(path).expect(&format!("File {} not found", path));
    let mut reader = BufReader::new(f);
    let mut buf = [0u8; PROGRAM_SIZE];

    reader
        .read(&mut buf)
        .expect(&format!("File {} cannot read", path));

    emu.memory[PROGRAM_HEAD..PROGRAM_HEAD + PROGRAM_SIZE].copy_from_slice(&buf);

    while emu.eip < MEMORY_SIZE as u32 {
        let code = emu.get_code8(0);
        if !matches.is_present("quiet") {
            println!("EIP = {:X}, Code = {:>02X}", emu.eip, code);
        }

        if let Some(f) = functions[code as usize] {
            f(&mut emu);
        } else {
            println!("\n\nNot Implemented: {:>02X}", code);
            break;
        }

        if emu.eip == 0 {
            println!("\n\nend of program.\n");
            break;
        }
    }

    emu.dump_registers();
}
