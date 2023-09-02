use std::{
    env::args,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};

const OP_MOV: u8 = 0b100010;

fn main() {
    let path: PathBuf = args().skip(1).take(1).map(PathBuf::from).collect();
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("could not open file");

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 2];

    println!("bits 16");
    println!();

    while let Ok(_) = reader.read_exact(&mut buffer) {
        let byte_1 = buffer[0];
        let byte_2 = buffer[1];

        let op_code = byte_1 >> 2; // opcode
        let d = (byte_1 >> 6) & 1; // direction is to register / direction is from register
        let w = byte_1 & 1; // word/byte operation

        let reg_mode = byte_2 >> 6; // register mode / memory mode
        let dest = (byte_2 >> 3) & 7; // register operand / dest
        let source = byte_2 & 7; // register operand / source

        if cfg!(debug_assertions) {
            println!("############# DEBUG ##############");
            println!("{byte_1:b} {byte_2:b}");
            println!("{byte_1:b} {byte_2:b}");
            println!("opcode : {op_code:b}");
            println!("d : {d:b}");
            println!("w : {w:b}");
            println!("reg_mod : {reg_mode:b}");
            println!("reg: {dest:b}");
            println!("rm: {source:b}");
            println!("############# DEBUG ##############");
        }

        if op_code == OP_MOV {
            print!("mov ");
        } else {
            panic!("opcoded {op_code:b} not implemented");
        }
        print!("{}, {}", register_table(source, w), register_table(dest, w));
        println!();
    }
}

fn register_table(register: u8, w: u8) -> &'static str {
    match (register, w) {
        (0, 0) => "al",
        (0, 1) => "ax",
        (1, 0) => "cl",
        (1, 1) => "cx",
        (0b10, 0) => "dl",
        (0b10, 1) => "dx",
        (0b11, 0) => "bl",
        (0b11, 1) => "bx",
        (0b100, 0) => "ah",
        (0b100, 1) => "sp",
        (0b101, 0) => "ch",
        (0b101, 1) => "bp",
        (0b110, 0) => "dh",
        (0b110, 1) => "si",
        (0b111, 0) => "bh",
        (0b111, 1) => "di",
        _ => panic!("{w} & {register} not implemented"),
    }
}
