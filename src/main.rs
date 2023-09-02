use std::{
    env::args,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
    usize,
};

const OP_MOV: u8 = 0b100010;
const REGISTER_TO_REGISTER: u8 = 0b11;

const REGISTERS: [&str; 16] = [
    "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di",
];

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
                println!("opcode : {op_code:b}");
                println!("d : {d:b}");
                println!("w : {w:b}");
                println!("reg_mod : {reg_mode:b}");
            }

            if reg_mode == REGISTER_TO_REGISTER {
                let reg = (byte_2 >> 3) & 7;
                let rm = byte_2 & 7;
                if cfg!(debug_assertions) {
                    println!("reg: {reg:b}");
                    println!("rm: {rm:b}");
                }
                let reg = REGISTERS[add_two_bytes(reg, w) as usize]; // register operand / source
                let rm = REGISTERS[add_two_bytes(rm, w) as usize]; // register operand / dest

                // if d is 0, instruction source is specified in reg field, else if d is 1 in rm field
                let (source, dest) = if d == 0 { (reg, rm) } else { (rm, reg) };
                println!("mov {dest}, {source}");
            }
        }
    }
}

#[inline]
fn add_two_bytes(high: u8, low: u8) -> u8 {
    (high << 0b1) | low
}
