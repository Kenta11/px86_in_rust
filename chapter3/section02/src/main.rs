use std::fs::File;
use std::io::{BufReader, Read};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use variant_count::VariantCount;

#[derive(Clone, Copy, Debug, EnumIter, VariantCount)]
enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

struct Emulator {
    registers: [u32; Register::VARIANT_COUNT],
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator {
            registers: [0; Register::VARIANT_COUNT],
            memory: vec![0; size],
            eip: eip,
        };

        emu.registers[Register::ESP as usize] = esp;

        emu
    }

    fn dump_registers(&self) {
        for r in Register::iter() {
            println!("{:?} = {:>08x}", &r, self.registers[r as usize]);
        }

        println!("EIP = {:>08X}", self.eip);
    }

    fn get_code8(&self, index: i32) -> u8 {
        self.memory[(self.eip + index as u32) as usize]
    }

    fn get_sign_code8(&self, index: i32) -> i8 {
        self.memory[(self.eip + index as u32) as usize] as i8
    }

    fn get_code32(&self, index: i32) -> u32 {
        let mut ret = 0u32;

        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }

        ret
    }

    fn get_sign_code32(&self, index: i32) -> i32 {
        self.get_code32(index) as i32
    }

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let value = self.get_code32(1);

        self.registers[reg as usize] = value;
        self.eip += 5;
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1) as u32;
        self.eip = (self.eip + 5).wrapping_add(diff);
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1) as u32;
        self.eip = (self.eip + 2).wrapping_add(diff);
    }
}

type InstructionFunctions = [Option<fn(&mut Emulator)>; 256];

trait New {
    fn new() -> InstructionFunctions;
}

impl New for InstructionFunctions {
    fn new() -> InstructionFunctions {
        let mut functions: InstructionFunctions = [None; 256];

        for f in functions.iter_mut() {
            *f = None;
        }

        for i in 0..8 {
            functions[0xB8 + i] = Some(Emulator::mov_r32_imm32);
        }
        functions[0xE9] = Some(Emulator::near_jump);
        functions[0xEB] = Some(Emulator::short_jump);

        functions
    }
}

fn main() {
    const MEMORY_SIZE: usize = 1_000_000;
    const PROGRAM_HEAD: usize = 0x7C00;
    const PROGRAM_SIZE: usize = 512;

    let path = std::env::args().nth(1).expect("Usage: px86 filename");

    let mut emu = Emulator::new(MEMORY_SIZE, PROGRAM_HEAD as u32, PROGRAM_HEAD as u32);

    let functions = InstructionFunctions::new();

    let f = File::open(&path).expect(&format!("File {} not found", &path));
    let mut reader = BufReader::new(f);
    let mut buf = [0u8; PROGRAM_SIZE];

    reader
        .read(&mut buf)
        .expect(&format!("File {} cannot read", path));

    emu.memory[PROGRAM_HEAD..PROGRAM_HEAD + PROGRAM_SIZE].copy_from_slice(&buf);

    while emu.eip < MEMORY_SIZE as u32 {
        let code = emu.get_code8(0);
        println!("EIP = 0x{:08X}, Code = {:>02X}", emu.eip, code);

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
