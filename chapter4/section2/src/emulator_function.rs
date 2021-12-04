use crate::emulator::{Emulator, Register32};

use strum::IntoEnumIterator;

enum Eflag {
    Carry,
    Zero,
    Sign,
    Overflow,
}

impl Eflag {
    fn map_to_u16(&self) -> u16 {
        match self {
            Eflag::Carry => 1,
            Eflag::Zero => 1 << 6,
            Eflag::Sign => 1 << 7,
            Eflag::Overflow => 1 << 11,
        }
    }
}

impl Emulator {
    pub fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator {
            registers: [0; Register32::VARIANT_COUNT],
            eflags: 0,
            memory: vec![0; size],
            eip: eip,
        };

        emu.registers[Register32::ESP as usize] = esp;

        emu
    }

    pub fn dump_registers(&self) {
        for r in Register32::iter() {
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

    pub fn get_register8(&self, index: i32) -> u8 {
        if (0 <= index) && (index < 4) {
            (self.registers[index as usize] & 0xff) as u8
        } else if (4 <= index) && (index < 8) {
            ((self.registers[(index - 4) as usize] >> 8) & 0xff) as u8
        } else {
            panic!()
        }
    }

    pub fn set_register8(&mut self, index: i32, value: u8) {
        if (0 <= index) && (index < 4) {
            let r = self.registers[index as usize] & 0xffffff00;
            self.registers[index as usize] = r | (value as u32);
        } else if (4 <= index) && (index < 8) {
            let r = self.registers[(index - 4) as usize] & 0xffff00ff;
            self.registers[(index - 4) as usize] = r | ((value as u32) << 8);
        } else {
            panic!()
        }
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

    pub fn push32(&mut self, value: u32) {
        let address = self.get_register32(Register32::ESP as i32) - 4;
        self.set_register32(Register32::ESP as i32, address);
        self.set_memory32(address, value);
    }

    pub fn pop32(&mut self) -> u32 {
        let address = self.get_register32(Register32::ESP as i32);
        let ret = self.get_memory32(address);
        self.set_register32(Register32::ESP as i32, address + 4);

        ret
    }

    pub fn set_carry(&mut self, is_carry: bool) {
        if is_carry {
            self.eflags |= Eflag::map_to_u16(&Eflag::Carry);
        } else {
            self.eflags &= !Eflag::map_to_u16(&Eflag::Carry);
        }
    }

    pub fn set_zero(&mut self, is_zero: bool) {
        if is_zero {
            self.eflags |= Eflag::map_to_u16(&Eflag::Zero);
        } else {
            self.eflags &= !Eflag::map_to_u16(&Eflag::Zero);
        }
    }

    pub fn set_sign(&mut self, is_sign: bool) {
        if is_sign {
            self.eflags |= Eflag::map_to_u16(&Eflag::Sign);
        } else {
            self.eflags &= !Eflag::map_to_u16(&Eflag::Sign);
        }
    }

    pub fn set_overflow(&mut self, is_overflow: bool) {
        if is_overflow {
            self.eflags |= Eflag::map_to_u16(&Eflag::Overflow);
        } else {
            self.eflags &= !Eflag::map_to_u16(&Eflag::Overflow);
        }
    }

    pub fn is_carry(&self) -> bool {
        (self.eflags & Eflag::map_to_u16(&Eflag::Carry)) == Eflag::map_to_u16(&Eflag::Carry)
    }

    pub fn is_zero(&self) -> bool {
        (self.eflags & Eflag::map_to_u16(&Eflag::Zero)) == Eflag::map_to_u16(&Eflag::Zero)
    }

    pub fn is_sign(&self) -> bool {
        (self.eflags & Eflag::map_to_u16(&Eflag::Sign)) == Eflag::map_to_u16(&Eflag::Sign)
    }

    pub fn is_overflow(&self) -> bool {
        (self.eflags & Eflag::map_to_u16(&Eflag::Overflow)) == Eflag::map_to_u16(&Eflag::Overflow)
    }

    pub fn update_eflags_sub(&mut self, v1: u32, v2: u32, result: u64) {
        let sign1 = (v1 >> 31) != 0;
        let sign2 = (v2 >> 31) != 0;
        let signr = ((result >> 31) & 1) != 0;

        self.set_carry((result >> 32) != 0);
        self.set_zero(result == 0);
        self.set_sign(!signr);
        self.set_overflow((sign1 != sign2) && (sign1 != signr));
    }
}
