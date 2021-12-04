mod modrm;

use crate::emulator::Emulator;
use modrm::ModRM;

impl Emulator {
    fn add_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        self.set_rm32(&modrm, rm32 + r32);
    }

    fn sub_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0) as i32;
        self.eip += 1;
        self.set_rm32(modrm, rm32 - imm8 as u32);
    }

    fn code_83(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();

        match unsafe { modrm.opereg.opecode } {
            5 => {
                self.sub_rm32_imm8(&modrm);
            },
            _ => {
                unimplemented!("not implemented: 83 {}", unsafe { modrm.opereg.opecode });
            },
        };
    }

    fn mov_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        self.set_rm32(&modrm, r32);
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
            },
            _ => {
                unimplemented!("not implemented: FF {}", unsafe { modrm.opereg.opecode });
            },
        };
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
        functions[0x83] = Some(Emulator::code_83);
        functions[0x89] = Some(Emulator::mov_rm32_r32);
        functions[0x8B] = Some(Emulator::mov_r32_rm32);
        for i in 0..8 {
            functions[0xB8 + i] = Some(Emulator::mov_r32_imm32);
        }
        functions[0xC7] = Some(Emulator::mov_rm32_imm32);
        functions[0xE9] = Some(Emulator::near_jump);
        functions[0xEB] = Some(Emulator::short_jump);
        functions[0xFF] = Some(Emulator::code_ff);

        functions
    }
}
