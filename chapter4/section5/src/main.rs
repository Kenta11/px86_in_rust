mod assembly;
mod fat;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufReader, Read};

const FILE_SIZE: usize = 512;

fn main() {
    let matches = App::new("boots")
        .version("1.0.0")
        .author("Kenta11")
        .arg(
            Arg::with_name("asm")
                .long("asm")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("filename")
                .value_name("FILE")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    
    let path = matches.value_of("filename").unwrap();
    let f = File::open(path).expect(&format!("File {} cannot be opened.", path));
    let mut reader = BufReader::new(f);
    let mut buf = [0u8; FILE_SIZE];

    reader
        .read(&mut buf)
        .expect(&format!("File {} cannot read", path));

    if matches.is_present("asm") {
        assembly::print_code(&buf);
    } else {
        fat::print_pbr(&buf);
    }
}
