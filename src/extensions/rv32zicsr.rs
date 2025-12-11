use crate::cpu;
use crate::extensions::Execute;
use crate::trap;

#[derive(Debug)]
pub enum RV32ZicsrInstruction {
    Csrrw(u8, u8, u16),
    Csrrs(u8, u8, u16),
    Csrrc(u8, u8, u16),
    Csrrwi(u8, u8, u16),
    Csrrsi(u8, u8, u16),
    Csrrci(u8, u8, u16),
}

impl Execute for RV32ZicsrInstruction {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap> {
        match self {
            RV32ZicsrInstruction::Csrrw(rd, rs1, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap(); // temporary error handling
                let data: u32 = cpu.regs.read(rs1);
                cpu.write_csr(csr, data)?; // temporary error handling
                cpu.regs.write(rd, t);
                return None;
            },
            RV32ZicsrInstruction::Csrrs(rd, rs1, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                let data: u32 = t | cpu.regs.read(rs1);
                cpu.write_csr(csr, data)?;
                cpu.regs.write(rd, t);
                return None;
            },
            RV32ZicsrInstruction::Csrrc(rd, rs1, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                let data: u32 = t & !cpu.regs.read(rs1);
                cpu.write_csr(csr, data)?;
                cpu.regs.write(rd, t);
                return None;
            },
            RV32ZicsrInstruction::Csrrwi(rd, zimm, csr) => {
                let data: u32 = cpu.read_csr(csr).unwrap();
                cpu.regs.write(rd, data);
                cpu.write_csr(csr, (zimm & 0x1F) as u32)?;
                return None;
            },
            RV32ZicsrInstruction::Csrrsi(rd, zimm, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                let data: u32 = t | (zimm & 0x1F) as u32;
                cpu.write_csr(csr, data)?;
                cpu.regs.write(rd, t);
                return None;
            },
            RV32ZicsrInstruction::Csrrci(rd, zimm, csr) => {
                let t: u32 = cpu.read_csr(csr).unwrap();
                let data: u32 = t & !(zimm & 0x1F) as u32;
                cpu.write_csr(csr, data)?;
                cpu.regs.write(rd, t);
                return None;
            },
        }
    }
}
