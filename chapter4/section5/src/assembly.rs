use crate::FILE_SIZE;
use byteorder::{ByteOrder, LittleEndian};

pub fn print_code(buffer: &[u8; FILE_SIZE]) {
    let fat_type = match LittleEndian::read_u16(&buffer[17..]) {
        0 => 32,
        512 => 16,
        _ => 12,
    };

    match fat_type {
        32 => print_assembly_code_for_fat32(&buffer),
        _ => print_assembly_code_for_fat16(&buffer),
    }
}

macro_rules! printdb {
    ($buffer:expr, $index:expr) => {
        println!("    db {}", $buffer[$index] as u8);
    }
}

macro_rules! printdw {
    ($buffer:expr, $index:expr) => {
        println!("    dw {}", LittleEndian::read_u16(&$buffer[$index..]));
    }
}

macro_rules! printdd {
    ($buffer:expr, $index:expr) => {
        println!("    dd {}", LittleEndian::read_u32(&$buffer[$index..]));
    }
}

macro_rules! print_string {
    ($buffer:expr, $index:expr, $len:expr) => {
        print!("    db \"");
        for i in 0..$len {
            print!("{}", $buffer[$index + i] as char);
        }
        println!("");
    }
}

fn print_assembly_code_for_fat32(buffer: &[u8; FILE_SIZE]) {
    print_bpb_structure(&buffer);

    printdd!(buffer, 36);
    printdw!(buffer, 40);
    printdw!(buffer, 42);
    printdd!(buffer, 44);
    printdw!(buffer, 48);
    printdw!(buffer, 50);
    println!("    times 64 - ($ - $$) db 0");
    printdb!(buffer, 64);
    printdb!(buffer, 65);
    printdb!(buffer, 66);
    printdd!(buffer, 67);
    print_string!(buffer, 71, 11);
    print_string!(buffer, 82, 8); 
    println!("entry:");
    println!("");
    println!("    times 510 - ($ - $$) db 0");
    println!("    db 0x55, 0xaa");
}

fn print_assembly_code_for_fat16(buffer: &[u8; FILE_SIZE]) {
    print_bpb_structure(&buffer);

    printdd!(buffer, 36);
    printdd!(buffer, 37);
    printdd!(buffer, 38);
    printdd!(buffer, 39);
    print_string!(buffer, 43, 11);
    print_string!(buffer, 54, 8); 
    println!("entry:");
    println!("");
    println!("    times 510 - ($ - $$) db 0");
    println!("    db 0x55, 0xaa");
}

fn print_bpb_structure(buffer: &[u8; FILE_SIZE]) {
    if buffer[0] == 0xebu8 && buffer[2] == 0x90u8 {
        println!("    jmp short entry");
        println!("    nop");
    } else if buffer[0] == 0xebu8 {
        println!("    jmp entry");
    } else {
        panic!("Unknown opecode: {:02x}", buffer[0]);
    }

    print_string!(buffer, 3, 8);
    printdw!(buffer, 11);
    printdb!(buffer, 13);
    printdw!(buffer, 14);
    printdb!(buffer, 16);
    printdw!(buffer, 17);
    printdw!(buffer, 19);
    printdb!(buffer, 21);
    printdw!(buffer, 22);
    printdw!(buffer, 24);
    printdw!(buffer, 26);
    printdd!(buffer, 28);
    printdd!(buffer, 32);
}
