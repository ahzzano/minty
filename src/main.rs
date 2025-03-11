use std::env;
use std::fs;

use compiler::Compiler;

pub mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let mips = compiler::mips::MIPS::default();

    let _ = mips.write_bin_file(vec![0x5544, 0x1233]);

    match command.as_str() {
        "run" => {}
        "compile" => {
            println!("Compiling file");
            let fname = &args[2];
            let contents = fs::read_to_string(fname).expect("No file found");
        }
        _ => {
            println!("Hello");
        }
    }

    println!("{command}");
}
