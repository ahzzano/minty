use std::collections::HashMap;

use super::Compiler;

#[derive(Debug, Default)]
pub struct MIPS {
    status: u32,
    // labels: Vec<(String, usize)>,
    labels: HashMap<String, usize>,
}

impl Compiler for MIPS {
    fn compile_file(&mut self, code: String) {
        let lines = code.split('\n');
        let mut to_out: Vec<u32> = Vec::new();

        for (ind, i) in lines.enumerate() {
            if let Some(label) = i.strip_suffix(":") {
                self.labels.insert(label.to_string(), ind);
                continue;
            }

            let inst = self.convert_instruction(i, ind);
            println!("{inst:?}");

            if let Some(inst) = inst {
                to_out.push(inst);
            }
        }

        self.write_bin_file(to_out).unwrap();
    }

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
    fn convert_instruction(&self, inst: &str, offset: usize) -> Option<u32> {
        if inst.contains(':') || inst.contains('.') {
            return None;
        }

        if inst.starts_with(";") || inst.starts_with("#") {
            return None;
        }

        if inst.is_empty() {
            return None;
        }

        let (inst, regs) = inst.trim().split_once(" ").unwrap();
        let (regs, _) = if regs.contains(";") {
            regs.trim().split_once(";").unwrap()
        } else {
            (regs, "")
        };
        let regs = regs.trim();

        match inst.trim() {
            "add" | "addu" | "sub" | "subu" | "or" | "nor" | "and" => {
                let reg: Vec<&str> = regs.trim().split(' ').collect();
                let rd = reg[0].trim().replace(',', "");
                let rs = reg[1].trim().replace(',', "");
                let rt = reg[2].trim().replace(',', "");

                let operation = match inst {
                    "add" => 0x20,
                    "addu" => 0x21,
                    "sub" => 0x22,
                    "subu" => 0x23,
                    "or" => 0x25,
                    "nor" => 0x27,
                    "and" => 0x24,
                    _ => 0,
                };

                let rd = self.convert_register_id(&rd).unwrap() << 11;
                let rt = self.convert_register_id(&rt).unwrap() << 16;
                let rs = self.convert_register_id(&rs).unwrap() << 21;

                let res = (rs | rd | rt) | operation;

                Some(res)
            }
            "addi" | "addiu" | "andi" | "ori" => {
                let opcode = match inst {
                    "addi" => 0x8,
                    "addiu" => 0x9,
                    "andi" => 0xc,
                    "ori" => 0xd,
                    _ => 0,
                } << 26;

                let regs: Vec<&str> = regs.trim().split(' ').collect();
                let rt = regs[0].trim().replace(",", "");
                let rs = regs[1].trim().replace(",", "");

                let mut imm = regs[2].trim().replace(",", "");

                let rt = self.convert_register_id(&rt).unwrap() << 16;
                let rs = self.convert_register_id(&rs).unwrap() << 21;

                let sign = if let Some(value) = imm.strip_prefix("-") {
                    imm = value.to_string();
                    -1
                } else {
                    1
                };

                let imm: i32 = if let Some(value) = imm.strip_prefix("0x") {
                    i32::from_str_radix(value, 16).unwrap()
                } else {
                    imm.parse().unwrap()
                } * sign;

                let imm = imm & 0x0000FFFF;

                Some(opcode | rt | rs | imm as u32)
            }
            "beq" | "bne" => {
                let opcode = match inst {
                    "beq" => 0x4,
                    "bne" => 0x5,
                    _ => 0,
                } << 26;

                let regs: Vec<&str> = regs.trim().split(' ').collect();

                let rt = regs[0].trim().replace(",", "");
                let rs = regs[1].trim().replace(",", "");
                let label = regs[2].trim().replace(",", "");

                let rt = self.convert_register_id(&rt).unwrap() << 16;
                let rs = self.convert_register_id(&rs).unwrap() << 21;

                let label_addr = self.labels.get(&label);
                assert!(label_addr.is_some());

                let label_offset: i32 = *label_addr.unwrap() as i32;

                Some(opcode | rt | rs | label_offset as u32)
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

        let eq = mips.convert_instruction("add $a0, $a1, $a2", 0);
        assert!(eq.is_some());
        assert_eq!(eq.unwrap(), 0x00A62020);

        let eq = mips.convert_instruction("add $a0 $a1 $a2", 0);
        assert!(eq.is_some());
        assert_eq!(eq.unwrap(), 0x00A62020);
    }

    #[test]
    fn test_addi_assembler() {
        let mips = MIPS::default();

        let eq = mips.convert_instruction("addi $a0, $a1, 0xFFFF", 0);
        assert!(eq.is_some());

        if let Some(inst) = eq {
            assert_eq!(inst, 0x20A4FFFF);
        }
        let eq = mips.convert_instruction("addi $a0, $a1, 50", 0);
        assert!(eq.is_some());

        if let Some(inst) = eq {
            assert_eq!(inst, 0x20A40032);
        }
        let eq = mips.convert_instruction("addi $a0, $a1, -50", 0);
        assert!(eq.is_some());
        if let Some(inst) = eq {
            assert_eq!(inst, 0x20A4FFCE);
        }
    }

    #[test]
    fn test_beq_assembler() {
        let mut mips = MIPS::default();
        mips.labels.insert("nonon_jakuzure".to_string(), 100);

        let eq = mips.convert_instruction("beq $a0, $a1, nonon_jakuzure", 1);
        if let Some(inst) = eq {
            assert_eq!(inst, 0x10A40064);
        }
    }
}
