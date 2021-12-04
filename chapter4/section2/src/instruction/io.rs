use std::io;
use std::io::{stdout, Write};

pub fn io_in8(address: u16) -> u8 {
    match address {
        0x03f8 => {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Input error!");
            guess.chars().next().unwrap() as u8
        }
        _ => panic!(),
    }
}

pub fn io_out8(address: u16, value: u8) {
    match address {
        0x03f8 => {
            print!("{}", value as char);
            stdout().flush().unwrap();
        }
        _ => (),
    }
}
