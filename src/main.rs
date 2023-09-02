use std::{
    env::args,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};

const OP_MOV: u8 = 0b100010;
const REGISTER_TO_REGISTER: u8 = 0b11;

fn main() {
    let path: PathBuf = args().skip(1).take(1).map(PathBuf::from).collect();
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("could not open file");

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 1];

    println!("bits 16");
    println!();

    while let Ok(_) = reader.read_exact(&mut buffer) {
        let byte_1 = buffer[0];

        let op_code = byte_1 >> 2; // opcode

        if op_code == OP_MOV {
            let d = (byte_1 >> 6) & 1; // direction is to register / direction is from register
            let w = byte_1 & 1; // word/byte operation
            reader
                .read_exact(&mut buffer)
                .expect("op mov requires an extra byte");
            let byte_2 = buffer[0];
            let reg_mode = byte_2 >> 6; // register mode / memory mode

            if cfg!(debug_assertions) {
                println!("############# DEBUG ##############");
                println!("{byte_1:b} {byte_2:b}");
                println!("{byte_1:b} {byte_2:b}");
                println!("opcode : {op_code:b}");
                println!("d : {d:b}");
                println!("w : {w:b}");
                println!("reg_mod : {reg_mode:b}");
            }

            if reg_mode == REGISTER_TO_REGISTER {
                let reg = register_table((byte_2 >> 3) & 7, w); // register operand / source
                let rm = register_table(byte_2 & 7, w); // register operand / dest
                if cfg!(debug_assertions) {
                    println!("reg: {reg}");
                    println!("rm: {rm}");
                }
                // it should be the opposite, but nasm always return 0
                // if d is 0, instruction source is specified in reg field, else if d is 1 in rm field
                let (source, dest) = if d == 0 { (reg, rm) } else { (rm, reg) };
                println!("mov {dest}, {source}");
            }
            if cfg!(debug_assertions) {
                println!("############# DEBUG ##############");
            }
        }
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
