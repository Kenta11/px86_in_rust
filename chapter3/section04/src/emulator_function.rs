use crate::emulator::{Emulator, Register};

use strum::IntoEnumIterator;

impl Emulator {
    pub fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator {
            registers: [0; Register::VARIANT_COUNT],
            memory: vec![0; size],
            eip: eip,
        };

        emu.registers[Register::ESP as usize] = esp;

        emu
    }

    pub fn dump_registers(&self) {
        for r in Register::iter() {
            println!("{:?} = {:>08x}", &r, self.registers[r as usize]);
        }

        println!("EIP = {:>08x}", self.eip);
    }

    pub fn get_code8(&self, index: i32) -> u8 {
        self.memory[(self.eip + index as u32) as usize]
    }

    pub fn get_sign_code8(&self, index: i32) -> i8 {
        self.memory[(self.eip + index as u32) as usize] as i8
    }

    pub fn get_code32(&self, index: i32) -> u32 {
        let mut ret = 0u32;

        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }

        ret
    }

    pub fn get_sign_code32(&self, index: i32) -> i32 {
        self.get_code32(index) as i32
    }

    pub fn get_register32(&self, index: i32) -> u32 {
        self.registers[index as usize]
    }

    pub fn set_register32(&mut self, index: i32, value: u32) {
        self.registers[index as usize] = value;
    }

    pub fn get_memory8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn get_memory32(&self, address: u32) -> u32 {
        let mut ret = 0u32;

        for offset in 0..4 {
            ret |= (self.get_memory8(address + offset) as u32) << (offset * 8);
        }

        ret
    }

    pub fn set_memory8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn set_memory32(&mut self, address: u32, value: u32) {
        for offset in 0..4 {
            self.set_memory8(address + offset, ((value >> (offset * 8)) & 0xFF) as u8);
        }
    }
}
