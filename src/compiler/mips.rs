use super::Compiler;

#[derive(Debug, Default)]
pub struct MIPS {
    status: u32,
}

impl Compiler for MIPS {
    fn compile_file(&self, code: String) {}
    fn convert_register_id(&self, reg: &str) -> Option<u32> {
        todo!()
    }
    fn convert_instruction(&self, inst: &str) -> Option<u32> {
        // label
        if inst.contains(':') {
            todo!()
        }

        let (inst, regs) = inst.split_once(' ').expect("Syntax error");

        match inst.trim() {
            "add" => {
                let opcode = 0x000;

                let reg: Vec<&str> = regs.trim().split(' ').collect();
                let rd = reg[0];
                let rs = reg[1];
                let rt = reg[2];

                let rd = self.convert_register_id(rd);
                let rs = self.convert_register_id(rs);
                let rt = self.convert_register_id(rt);

                Some(0x000)
            }
            _ => None,
        }
    }
}
