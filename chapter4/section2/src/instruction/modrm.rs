use crate::emulator::Emulator;

pub union OpeReg {
    pub opecode: u8,
    reg_index: u8,
}

pub union Disp {
    disp8: i8,
    disp32: u32,
}

pub struct ModRM {
    pub m: u8,
    pub opereg: OpeReg,
    rm: u8,
    sib: u8,
    disp: Disp,
}

impl Emulator {
    pub fn parse_modrm(&mut self) -> ModRM {
        let code = self.get_code8(0);
        let mut modrm = ModRM {
            m: (code & 0xC0) >> 6,
            opereg: OpeReg {
                opecode: (code & 0x38) >> 3,
            },
            rm: code & 0x07,
            sib: 0,
            disp: Disp { disp32: 0 },
        };

        self.eip += 1;

        if modrm.m != 3 && modrm.rm == 4 {
            modrm.sib = self.get_code8(0);
            self.eip += 1;
        }

        if (modrm.m == 0 && modrm.rm == 5) || modrm.m == 2 {
            modrm.disp.disp32 = self.get_code32(0);
            self.eip += 4;
        } else if modrm.m == 1 {
            modrm.disp.disp8 = self.get_sign_code8(0);
            self.eip += 1;
        }

        modrm
    }

    pub fn calc_memory_address(&self, modrm: &ModRM) -> u32 {
        match modrm.m {
            0 => match modrm.rm {
                4 => {
                    unimplemented!("not implemented ModRM mod = 0, rm = 4");
                }
                5 => unsafe { modrm.disp.disp32 },
                _ => self.get_register32(modrm.rm as u32 as i32),
            },
            1 => match modrm.m {
                4 => {
                    unimplemented!("not implemented ModRM mod = 1, rm = 4");
                }
                _ => self
                    .get_register32(modrm.rm as u32 as i32)
                    .wrapping_add(unsafe { modrm.disp.disp8 } as i32 as u32),
            },
            2 => match modrm.m {
                4 => {
                    unimplemented!("not implemented ModRM mod = 2, rm = 4");
                }
                _ => self.get_register32(modrm.rm as u32 as i32) + unsafe { modrm.disp.disp32 },
            },
            _ => {
                unimplemented!("not implemented ModRM mod = 3");
            }
        }
    }

    pub fn get_rm32(&self, modrm: &ModRM) -> u32 {
        if modrm.m == 3 {
            self.get_register32(modrm.rm as u32 as i32)
        } else {
            self.get_memory32(self.calc_memory_address(modrm))
        }
    }

    pub fn set_rm32(&mut self, modrm: &ModRM, value: u32) {
        if modrm.m == 3 {
            self.set_register32(modrm.rm as u32 as i32, value);
        } else {
            self.set_memory32(self.calc_memory_address(modrm), value);
        }
    }

    pub fn get_r32(&self, modrm: &ModRM) -> u32 {
        self.get_register32(unsafe { modrm.opereg.reg_index } as u32 as i32)
    }

    pub fn set_r32(&mut self, modrm: &ModRM, value: u32) {
        self.set_register32(unsafe { modrm.opereg.reg_index } as u32 as i32, value);
    }

    #[warn(dead_code)]
    pub fn set_rm8(&mut self, modrm: &ModRM, value: u8) {
        if modrm.m == 3 {
            self.set_register8(modrm.rm as i32, value);
        } else {
            let address = self.calc_memory_address(modrm);
            self.set_memory8(address, value);
        }
    }

    pub fn get_rm8(&mut self, modrm: &ModRM) -> u8 {
        if modrm.m == 3 {
            self.get_register8(modrm.rm as i32)
        } else {
            let address = self.calc_memory_address(modrm);
            self.get_memory8(address)
        }
    }

    pub fn set_r8(&mut self, modrm: &ModRM, value: u8) {
        self.set_register8(unsafe { modrm.opereg.reg_index } as i32, value);
    }

    #[warn(dead_code)]
    pub fn get_r8(&mut self, modrm: &ModRM) -> u8 {
        self.get_register8(unsafe { modrm.opereg.reg_index } as i32)
    }
}
