mod io;
mod modrm;
mod bios;

use crate::emulator::{Emulator, Register32, Register8};
use io::{io_in8, io_out8};
use modrm::ModRM;

impl Emulator {
    fn add_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        self.set_rm32(&modrm, rm32 + r32);
    }

    fn cmp_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();

        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        let result = (r32 as u64) - (rm32 as u64);
        self.update_eflags_sub(r32, rm32, result);
    }

    fn cmp_al_imm8(&mut self) {
        let value = self.get_code8(1);
        let al = self.get_register8(Register8::AL as i32);
        let result = (al as u64).wrapping_sub(value as u64);
        self.update_eflags_sub(al as u32, value as u32, result);
        self.eip += 2;
    }

    fn cmp_eax_imm32(&mut self) {
        let value = self.get_code32(1);
        let eax = self.get_register32(Register32::EAX as i32);
        let result = (eax as u64).wrapping_sub(value as u64);
        self.update_eflags_sub(eax, value, result);
        self.eip += 5;
    }

    fn inc_r32(&mut self) {
        let reg = self.get_code8(0) - 0x40;
        self.set_register32(reg as i32, self.get_register32(reg as i32) + 1);
        self.eip += 1;
    }

    fn sub_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0) as i32;
        self.eip += 1;
        self.set_rm32(modrm, rm32 - imm8 as u32);
    }

    fn code_83(&mut self) {
        self.eip += 1;
        let mut modrm = self.parse_modrm();

        match unsafe { modrm.opereg.opecode } {
            0 => {
                self.add_rm32_imm8(&mut modrm);
            }
            5 => {
                self.sub_rm32_imm8(&modrm);
            }
            _ => {
                unimplemented!("not implemented: 83 {}", unsafe { modrm.opereg.opecode });
            }
        };
    }

    fn mov_rm8_r8(&mut self) {
        let reg = self.get_code8(0) - 0x40;
        self.set_register32(reg as i32, self.get_register32(reg as i32) + 1);
        self.eip += 1;
    }

    fn mov_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        self.set_rm32(&modrm, r32);
    }

    fn mov_r8_rm8(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let rm8 = self.get_rm8(&modrm);
        self.set_r8(&modrm, rm8);
    }

    fn push_r32(&mut self) {
        let reg = self.get_code8(0) - 0x50;
        self.push32(self.get_register32(reg as i32));
        self.eip += 1;
    }

    fn pop_r32(&mut self) {
        let reg = self.get_code8(0) - 0x58;
        let value = self.pop32();
        self.set_register32(reg as u32 as i32, value);
        self.eip += 1;
    }

    fn push_imm32(&mut self) {
        let value = self.get_code32(1);
        self.push32(value);
        self.eip += 5;
    }

    fn push_imm8(&mut self) {
        let value = self.get_code8(1);
        self.push32(value as u32);
        self.eip += 2;
    }

    fn jo(&mut self) {
        let diff = if self.is_overflow() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jno(&mut self) {
        let diff = if self.is_overflow() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jc(&mut self) {
        let diff = if self.is_carry() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jnc(&mut self) {
        let diff = if self.is_carry() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jz(&mut self) {
        let diff = if self.is_zero() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jnz(&mut self) {
        let diff = if self.is_zero() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn js(&mut self) {
        let diff = if self.is_sign() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jns(&mut self) {
        let diff = if self.is_zero() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jl(&mut self) {
        let diff = if self.is_sign() != self.is_overflow() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn jle(&mut self) {
        let diff = if self.is_zero() || (self.is_sign() != self.is_overflow()) {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip = (self.eip + 2).wrapping_add(diff as i32 as u32);
    }

    fn swi(&mut self) {
        let int_index = self.get_code8(1);
        self.eip += 2;

        match int_index {
            0x10 => self.bios_video(),
            _ => println!("unknown interrupt: {:02x}", int_index),
        }
    }

    fn add_rm32_imm8(&mut self, modrm: &mut ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0) as i32;
        self.eip += 1;
        self.set_rm32(&modrm, rm32 + imm8 as u32);
    }

    fn mov_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let rm32 = self.get_rm32(&modrm);
        self.set_r32(&modrm, rm32);
    }

    fn mov_r8_imm8(&mut self) {
        let reg = self.get_code8(0) - 0xB0;
        self.set_register8(reg as i32, self.get_code8(1));
        self.eip += 2;
    }

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let value = self.get_code32(1);

        self.registers[reg as usize] = value;
        self.eip += 5;
    }

    fn mov_rm32_imm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let value = self.get_code32(0);
        self.eip += 4;
        self.set_rm32(&modrm, value);
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1) as u32;
        self.eip = self.eip.wrapping_add(diff).wrapping_add(5);
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1) as u32;
        self.eip = self.eip.wrapping_add(diff).wrapping_add(2);
    }

    fn in_al_dx(&mut self) {
        let address = (self.get_register32(Register32::EDX as i32) & 0xffff) as u16;
        let value = io_in8(address);
        self.set_register8(Register8::AL as i32, value);
        self.eip += 1;
    }

    fn out_dx_al(&mut self) {
        let address = (self.get_register32(Register32::EDX as i32) & 0xffff) as u16;
        let value = self.get_register8(Register8::AL as i32);
        io_out8(address, value);
        self.eip += 1;
    }

    fn inc_rm32(&mut self, modrm: &mut ModRM) {
        let value = self.get_rm32(&modrm);
        self.set_rm32(&modrm, value + 1);
    }

    fn code_ff(&mut self) {
        self.eip += 1;
        let mut modrm = self.parse_modrm();

        match unsafe { modrm.opereg.opecode } {
            0 => {
                self.inc_rm32(&mut modrm);
            }
            _ => {
                unimplemented!("not implemented: FF {}", unsafe { modrm.opereg.opecode });
            }
        };
    }

    fn call_ref32(&mut self) {
        let diff = self.get_sign_code32(1);
        self.push32(self.eip + 5);
        self.eip = self.eip.wrapping_add(diff as u32).wrapping_add(5);
    }

    fn ret(&mut self) {
        self.eip = self.pop32();
    }

    fn leave(&mut self) {
        let ebp = self.get_register32(Register32::EBP as i32);
        self.set_register32(Register32::ESP as i32, ebp);

        let value = self.pop32();
        self.set_register32(Register32::EBP as i32, value);
        self.eip += 1;
    }
}

pub type InstructionFunctions = [Option<fn(&mut Emulator)>; 256];

pub trait New {
    fn new() -> InstructionFunctions;
}

impl New for InstructionFunctions {
    fn new() -> InstructionFunctions {
        let mut functions: InstructionFunctions = [None; 256];

        for f in functions.iter_mut() {
            *f = None;
        }

        functions[0x01] = Some(Emulator::add_rm32_r32);
        functions[0x3B] = Some(Emulator::cmp_r32_rm32);
        functions[0x3C] = Some(Emulator::cmp_al_imm8);
        functions[0x3D] = Some(Emulator::cmp_eax_imm32);
        for i in 0..8 {
            functions[0x40 + i] = Some(Emulator::inc_r32);
        }
        for i in 0..8 {
            functions[0x50 + i] = Some(Emulator::push_r32);
        }
        for i in 0..8 {
            functions[0x58 + i] = Some(Emulator::pop_r32);
        }
        functions[0x68] = Some(Emulator::push_imm32);
        functions[0x6A] = Some(Emulator::push_imm8);
        functions[0x70] = Some(Emulator::jo);
        functions[0x71] = Some(Emulator::jno);
        functions[0x72] = Some(Emulator::jc);
        functions[0x73] = Some(Emulator::jnc);
        functions[0x74] = Some(Emulator::jz);
        functions[0x75] = Some(Emulator::jnz);
        functions[0x78] = Some(Emulator::js);
        functions[0x79] = Some(Emulator::jns);
        functions[0x7C] = Some(Emulator::jl);
        functions[0x7E] = Some(Emulator::jle);
        functions[0x83] = Some(Emulator::code_83);
        functions[0x88] = Some(Emulator::mov_rm8_r8);
        functions[0x89] = Some(Emulator::mov_rm32_r32);
        functions[0x8A] = Some(Emulator::mov_r8_rm8);
        functions[0x8B] = Some(Emulator::mov_r32_rm32);
        for i in 0..8 {
            functions[0xB0 + i] = Some(Emulator::mov_r8_imm8);
        }
        for i in 0..8 {
            functions[0xB8 + i] = Some(Emulator::mov_r32_imm32);
        }
        functions[0xC3] = Some(Emulator::ret);
        functions[0xC7] = Some(Emulator::mov_rm32_imm32);
        functions[0xC9] = Some(Emulator::leave);
        functions[0xCD] = Some(Emulator::swi);
        functions[0xE8] = Some(Emulator::call_ref32);
        functions[0xE9] = Some(Emulator::near_jump);
        functions[0xEB] = Some(Emulator::short_jump);
        functions[0xEC] = Some(Emulator::in_al_dx);
        functions[0xEE] = Some(Emulator::out_dx_al);
        functions[0xFF] = Some(Emulator::code_ff);

        functions
    }
}
