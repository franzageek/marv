use crate::cpu;
use crate::extensions::Execute;
use crate::trap;

pub enum RV32ZicsrInstruction {
    Csrrw(u8, u8, u16),
    Csrrs(u8, u8, u16),
    Csrrc(u8, u8, u16),
    Csrrwi(u8, u8, u16),
    Csrrsi(u8, u8, u16),
    Csrrci(u8, u8, u16),
}

impl std::fmt::Debug for RV32ZicsrInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Csrrw(rd, rs1, csr) => write!(f, "Csrrw {{ rd:x{rd}, rs1:x{rs1}, csr:{csr}|0x{:04X} }}", csr),
            Self::Csrrs(rd, rs1, csr) => write!(f, "Csrrs {{ rd:x{rd}, rs1:x{rs1}, csr:{csr}|0x{:04X} }}", csr),
            Self::Csrrc(rd, rs1, csr) => write!(f, "Csrrc {{ rd:x{rd}, rs1:x{rs1}, csr:{csr}|0x{:04X} }}", csr),
            Self::Csrrwi(rd, zimm, csr) => write!(f, "Csrrwi {{ rd:x{rd}, zimm:{zimm}, csr:{csr}|0x{:04X} }}", csr),
            Self::Csrrsi(rd, zimm, csr) => write!(f, "Csrrsi {{ rd:x{rd}, zimm:{zimm}, csr:{csr}|0x{:04X} }}", csr),
            Self::Csrrci(rd, zimm, csr) => write!(f, "Csrrci {{ rd:x{rd}, zimm:{zimm}, csr:{csr}|0x{:04X} }}", csr),
        }
    }
}

impl Execute for RV32ZicsrInstruction {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap> {
        match self {
            RV32ZicsrInstruction::Csrrw(rd, rs1, csr) => {
                if rd > 0 {
                    let t: u32 = cpu.read_csr(csr).unwrap(); // temporary error handling
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                let data: u32 = cpu.regs.read(rs1);
                //eprintln!("[data]:<0x{:08X}>", data);
                cpu.write_csr(csr, data)?; // temporary error handling
                //let t: u32 = cpu.read_csr(csr).unwrap();
                //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                return None;
            },
            RV32ZicsrInstruction::Csrrs(rd, rs1, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                if rd > 0 {
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                if rs1 > 0 {
                    let data: u32 = t | cpu.regs.read(rs1);
                    cpu.write_csr(csr, data)?;
                    //eprintln!("[data]:<0x{:08X}>", data);
                    //let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                }
                return None;
            },
            RV32ZicsrInstruction::Csrrc(rd, rs1, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                if rd > 0 {
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                if rs1 > 0 {
                    let data: u32 = t & !cpu.regs.read(rs1);
                    cpu.write_csr(csr, data)?;
                    //eprintln!("[data]:<0x{:08X}>", data);
                    //let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                }
                return None;
            },
            RV32ZicsrInstruction::Csrrwi(rd, zimm, csr) => {
                if rd > 0 {
                    let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                if zimm > 0 {
                    cpu.write_csr(csr, (zimm & 0x1F) as u32)?;
                    //eprintln!("[zimm]:<0x{:08X}>", (zimm & 0x1F));
                    //let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                }
                return None;
            },
            RV32ZicsrInstruction::Csrrsi(rd, zimm, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                if rd > 0 {
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                if zimm > 0 {
                    let data: u32 = t | (zimm & 0x1F) as u32;
                    cpu.write_csr(csr, data)?;
                    //eprintln!("[data]:<0x{:08X}>", data);
                    //let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                }
                return None;
            },
            RV32ZicsrInstruction::Csrrci(rd, zimm, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                if rd > 0 {
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                    cpu.regs.write(rd, t);
                }
                if zimm > 0 {
                    let data: u32 = t & !(zimm & 0x1F) as u32;
                    cpu.write_csr(csr, data)?;
                    //eprintln!("[data]:<0x{:08X}>", data);
                    //let t: u32 = cpu.read_csr(csr).unwrap();
                    //eprintln!("[csr<{csr}>]:<0x{:08X}>", t);
                }
                return None;
            },
        }
    }
}
