use crate::emulator::{Emulator, Register8};
use crate::instruction::io::io_out8;

const BIOS_TO_TERMINAL: [i32; 8] = [30, 34, 32, 36, 31, 35, 33, 37];

fn put_string(s: &str) {
    for c in s.as_bytes() {
        io_out8(0x03f8, *c);
    }
}

impl Emulator {
    fn bios_video_teletype(&mut self) {
        let color = self.get_register8(Register8::BL as i32) & 0x0f;
        let ch = self.get_register8(Register8::AL as i32);

        let terminal_color = BIOS_TO_TERMINAL[(color & 0x07) as usize];
        let bright = if (color & 0x08) == 0x08 {1} else {0};
        put_string(&format!("\x1b[{};{}m{}\x1b[0m", bright, terminal_color, ch as char));
    }

    pub fn bios_video(&mut self) {
        let func = self.get_register8(Register8::AH as i32);
        match func {
            0x0e => self.bios_video_teletype(),
            _ => println!("not implemented BIOS video function: 0x{:02x}", func),
        }
    }
}
