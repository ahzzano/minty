use super::Compiler;

#[derive(Debug, Default)]
pub struct MIPS {
    status: u32,
}

impl Compiler for MIPS {
    fn compile_file(&self, code: String) {}
    fn convert_register_id(&self, reg: &str) -> Option<u32> {
        match reg {
            "$0" | "$zero" | "0" | "zero" => Some(0),
            "$at" | "at" => Some(1),
            "$v0" | "v0" => Some(2),
            "$v1" | "v1" => Some(3),
            "$a0" | "a0" => Some(4),
            "$a1" | "a1" => Some(5),
            "$a2" | "a2" => Some(6),
            "$a3" | "a3" => Some(7),
            "$t0" | "t0" => Some(8),
            "$t1" | "t1" => Some(9),
            "$t2" | "t2" => Some(10),
            "$t3" | "t3" => Some(11),
            "$t4" | "t4" => Some(12),
            "$t5" | "t5" => Some(13),
            "$t6" | "t6" => Some(14),
            "$t7" | "t7" => Some(15),
            "$s0" | "s0" => Some(16),
            "$s1" | "s1" => Some(17),
            "$s2" | "s2" => Some(18),
            "$s3" | "s3" => Some(19),
            "$s4" | "s4" => Some(20),
            "$s5" | "s5" => Some(21),
            "$s6" | "s6" => Some(22),
            "$s7" | "s7" => Some(23),
            "$t8" | "t8" => Some(24),
            "$t9" | "t9" => Some(25),
            "$k0" | "k0" => Some(26),
            "$k1" | "k1" => Some(27),
            "$gp" | "gp" => Some(28),
            "$sp" | "sp" => Some(29),
            "$fp" | "fp" => Some(30),
            "$ra" | "ra" => Some(31),
            _ => None,
        }
    }
    fn convert_instruction(&self, inst: &str) -> Option<u32> {
        // label
        if inst.contains(':') {
            todo!()
        }

        let (inst, regs) = inst.split_once(' ').expect("Syntax error");
        println!("{inst}");

        match inst.trim() {
            "add" | "sub" => {
                let reg: Vec<&str> = regs.trim().split(' ').collect();
                let rd = reg[0].trim().replace(',', "");
                let rs = reg[1].trim().replace(',', "");
                let rt = reg[2].trim().replace(',', "");

                let operation = match inst {
                    "add" => 0x20,
                    "sub" => 0x22,
                    _ => 0,
                };

                println!("{rs} {rd} {rt} {reg:?}");

                let rd = self.convert_register_id(&rd).unwrap() << 11;
                let rt = self.convert_register_id(&rt).unwrap() << 16;
                let rs = self.convert_register_id(&rs).unwrap() << 21;

                let res = (rs | rd | rt) | 0x20;

                Some(res)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::Compiler;

    use super::MIPS;

    #[test]
    fn test_add_assembler() {
        let mips = MIPS::default();

        let eq = mips.convert_instruction("add $a0, $a1, $a2").unwrap();
        assert_eq!(eq, 0x00A62020);

        let eq = mips.convert_instruction("add $a0 $a1 $a2").unwrap();
        assert_eq!(eq, 0x00A62020);
    }
}
