use crate::FILE_SIZE;
use byteorder::{ByteOrder, LittleEndian};

pub fn print_pbr(buffer: &[u8; FILE_SIZE]) {
    let fat_type = match LittleEndian::read_u16(&buffer[17..]) {
        0 => 32,
        512 => 16,
        _ => 12,
    };

    println!("Calculated fat type is FAT{}", fat_type);

    print_bpb_structure(&buffer);

    match fat_type {
        32 => print_bss_structure_fat32(&buffer),
        _ => print_bss_structure_fat16(&buffer),
    }
}

fn print_bpb_structure(buffer: &[u8; FILE_SIZE]) {
    println!("{:<20} {:02x} {:02x} {:02x}", "BS_jmpBoot:", buffer[0] as u8, buffer[1] as u8, buffer[2] as u8);

    print!("{:<20} ", "BS_OEMName:");
    for i in 0..8 {
        print!("{}", buffer[i + 3] as char);
    }
    print!("\n");

    println!("{:<20} {}", "BPB_BytsPerSec:", LittleEndian::read_u16(&buffer[11..]));
    println!("{:<20} {}", "BPB_SecPerClus:", buffer[13] as u8);
    println!("{:<20} {}", "BPB_RsvdSecCnt:", LittleEndian::read_u16(&buffer[14..]));
    println!("{:<20} {}", "BPB_NumFATs:", buffer[16] as u8);
    println!("{:<20} {}", "BPB_RootEntCnt:", LittleEndian::read_u16(&buffer[17..]));
    println!("{:<20} {}", "BPB_TotSec16:", LittleEndian::read_u16(&buffer[19..]));
    println!("{:<20} {:02x}", "BPB_Media:", buffer[21] as u8);
    println!("{:<20} {}", "BPB_FATSz16:", LittleEndian::read_u16(&buffer[22..]));
    println!("{:<20} {}", "BPB_SecPerTrk:", LittleEndian::read_u16(&buffer[24..]));
    println!("{:<20} {}", "BPB_NumHeads:", LittleEndian::read_u16(&buffer[26..]));
    println!("{:<20} {}", "BPB_HiddSec:", LittleEndian::read_u32(&buffer[28..]));
    println!("{:<20} {}", "BPB_TotSec32:", LittleEndian::read_u32(&buffer[32..]));
}

fn print_bss_structure_fat16(buffer: &[u8; FILE_SIZE]) {
    println!("{:<20} {}", "BS_DrvNum:", buffer[36] as u8);
    println!("{:<20} {}", "BS_Reserved1:", buffer[37] as u8);
    println!("{:<20} {}", "BS_BootSig:", buffer[38] as u8);
    println!("{:<20} {}", "BS_VolID:", LittleEndian::read_u32(&buffer[39..]));

    print!("{:<20} ", "BS_VolLab:");
    for i in 0..11 {
        print!("{}", buffer[i + 43] as char);
    }
    println!("");

    print!("{:<20} ", "BS_FilSysType:");
    for i in 0..8 {
        print!("{}", buffer[i + 54] as char);
    }
    println!("");
}

fn print_bss_structure_fat32(buffer: &[u8; FILE_SIZE]) {
    println!("{:<20} {}", "BS_DrvNum:", buffer[64] as u8);
    println!("{:<20} {}", "BS_Reserved1:", buffer[65] as u8);
    println!("{:<20} {}", "BS_BootSig:", buffer[65] as u8);
    println!("{:<20} {}", "BS_VolID:", LittleEndian::read_u32(&buffer[67..]));

    print!("{:<20} ", "BS_VolLab:");
    for i in 0..11 {
        print!("{}", buffer[i + 71] as char);
    }
    println!("");

    print!("{:<20} ", "BS_FilSysType:");
    for i in 0..8 {
        print!("{}", buffer[i + 82] as char);
    }
    println!("");
}
