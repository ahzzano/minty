use std::{error::Error, fs, io::Write};

pub mod mips;

pub trait Compiler {
    fn compile_file(&self, code: String);

    fn convert_instruction(&self, inst: &str) -> Option<u32>;
    fn convert_register_id(&self, reg: &str) -> Option<u32>;

    fn write_bin_file(&self, instructions: Vec<u32>) -> Result<(), std::io::Error> {
        let mut file = fs::File::create("a.bin")?;

        let bytes: Vec<u8> = {
            let length_u32 = instructions.len();
            // u32 - 32 bits
            // u8  - 8 bits
            //
            // s(u32) / s(u8) = 4
            let new_length = length_u32 * 4;
            let mut out = vec![0; new_length];

            let mut addr = 0;

            for i in instructions {
                let a: u8 = (i & 0x000000FF) as u8;
                let b: u8 = ((i >> 8) & 0x000000FF) as u8;
                let c: u8 = ((i >> 16) & 0x000000FF) as u8;
                let d: u8 = ((i >> 24) & 0x000000FF) as u8;

                out[addr] = a;
                out[addr + 1] = b;
                out[addr + 2] = c;
                out[addr + 3] = d;
                addr += 4;
            }

            out
        };

        let _ = file.write(bytes.as_slice())?;

        Ok(())
    }
}

// pub fn compile_file(code: String) -> Vec<u32> {
//     let instructions: Vec<u32> = Vec::new();
//
//     for i in code.split('\n') {
//         println!("{i}");
//         convert_instruction(i);
//     }
//
//     instructions
// }
//
// fn convert_instruction(str: &str) -> u32 {
//     todo!()
// }
