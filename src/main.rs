use std::{
    env::args,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
    usize,
};

const OP_MOV: u8 = 0b100010;
const OP_IMMEDIATE_TO_REGISTER: u8 = 0b1011;
const REGISTER_TO_REGISTER_MODE: u8 = 0b11;
const MEMORY_MODE: u8 = 0;
const _MEMORY_MODE_EIGHT_BITS_DISPLACEMENT: u8 = 1;
const MEMORY_MODE_SIXTEEN_BITS_DISPLACEMENT: u8 = 2;

const REGISTERS: [&str; 16] = [
    "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di",
];

const EFFECTIVE_ADDR_CALCULATION_MOD: [&str; 8] = [
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
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

    while reader.read_exact(&mut buffer).is_ok() {
        let byte_1 = buffer[0];

        if byte_1 >> 2 == OP_MOV {
            let d = (byte_1 >> 1) & 0b1; // direction is to register / direction is from register

            let w = byte_1 & 1; // word/byte operation

            reader
                .read_exact(&mut buffer)
                .expect("op mov requires an extra byte");
            let byte_2 = buffer[0];
            let reg_mode = byte_2 >> 6; // register mode / memory mode
            let reg = (byte_2 >> 3) & 7;
            let rm = byte_2 & 7;

            //   let (reg, rm) = if d == 0 { (reg, rm) } else { (rm, reg) };
            if cfg!(debug_assertions) {
                println!("############# DEBUG ##############");
                println!("{byte_1:b} {byte_2:b}");
                println!("d : {d:b}");
                println!("w : {w:b}");
                println!("reg_mod : {reg_mode:b}");
                println!("reg : {reg:b}");
                println!("rm : {rm:b}");
                println!("############# DEBUG ##############");

                println!();
            }
            if reg_mode == REGISTER_TO_REGISTER_MODE {
                let reg = REGISTERS[add_two_bytes(reg, w) as usize]; // register operand / source
                let rm = REGISTERS[add_two_bytes(rm, w) as usize]; // register operand / dest
                if d == 0 {
                    println!("mov {rm}, {reg}",);
                } else {
                    println!("mov {reg}, {rm}",);
                }
            } else if reg_mode < 0b11 {
                let mut acc = 0u16;

                if reg_mode > 0 || rm == 0b110 {
                    reader
                        .read_exact(&mut buffer)
                        .expect("op mov memory mode 8bits displacement requires an extra byte");
                    let disp_lo = buffer[0];
                    acc = disp_lo as u16;
                }

                let source = REGISTERS[add_two_bytes(reg, w) as usize]; // register operand / source

                if reg_mode == MEMORY_MODE_SIXTEEN_BITS_DISPLACEMENT
                    || (reg_mode == MEMORY_MODE && rm == 0b110)
                {
                    reader
                        .read_exact(&mut buffer)
                        .expect("op mov memory mode 16bits displacement requires two extra bytes");
                    let disp_hi = buffer[0];
                    acc |= (disp_hi as u16) << 8;
                }
                let mut dest = EFFECTIVE_ADDR_CALCULATION_MOD[rm as usize].to_string();
                if acc > 0 {
                    dest = format!("{dest} + {acc}");
                }

                if d == 0 {
                    println!("mov [{dest}], {source}",);
                } else {
                    println!("mov {source}, [{dest}]",);
                }
            }
        } else if byte_1 >> 4 == OP_IMMEDIATE_TO_REGISTER {
            let w = (byte_1 & 0b00001000) >> 3;
            let reg = byte_1 & 0b00000111;
            reader
                .read_exact(&mut buffer)
                .expect("immediate to register requires one extra bytes");
            let data = buffer[0];
            let mut acc = data as u16;
            if w == 1 {
                reader
                    .read_exact(&mut buffer)
                    .expect("immediate to register requires two extra bytes");
                acc |= (buffer[0] as u16) << 8;
            }
            let reg = REGISTERS[add_two_bytes(reg, w) as usize]; // register operand / source
            println!("mov {reg}, {acc}");
        }
    }
}

#[inline]
fn add_two_bytes(high: u8, low: u8) -> u8 {
    (high << 0b1) | low
}
