mod modrm;

use crate::emulator::{Emulator, Register};
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

    fn mov_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        self.set_rm32(&modrm, r32);
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
        let ebp = self.get_register32(Register::EBP as i32);
        self.set_register32(Register::ESP as i32, ebp);

        let value = self.pop32();
        self.set_register32(Register::EBP as i32, value);
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
        functions[0x89] = Some(Emulator::mov_rm32_r32);
        functions[0x8B] = Some(Emulator::mov_r32_rm32);
        for i in 0..8 {
            functions[0xB8 + i] = Some(Emulator::mov_r32_imm32);
        }
        functions[0xC3] = Some(Emulator::ret);
        functions[0xC7] = Some(Emulator::mov_rm32_imm32);
        functions[0xC9] = Some(Emulator::leave);
        functions[0xE8] = Some(Emulator::call_ref32);
        functions[0xE9] = Some(Emulator::near_jump);
        functions[0xEB] = Some(Emulator::short_jump);
        functions[0xFF] = Some(Emulator::code_ff);

        functions
    }
}
