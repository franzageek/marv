use crate::cpu;
use crate::extensions::Execute;
use crate::trap;

#[allow(dead_code)]
#[derive(Debug)] // [ ] implement debug
pub enum RV32AInstruction { // temporary implementation
    ScW(u8, u8, u8),
    AmoswapW(u8, u8, u8),
    AmoaddW(u8, u8, u8),
    AmoxorW(u8, u8, u8),
    AmoandW(u8, u8, u8),
    AmoorW(u8, u8, u8),
    AmominW(u8, u8, u8),
    AmomaxW(u8, u8, u8),
    AmominuW(u8, u8, u8),
    AmomaxuW(u8, u8, u8),
}

impl Execute for RV32AInstruction {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap> {
        match self {
            RV32AInstruction::ScW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                cpu.mem.write_word(address as usize, cpu.regs.read(rs2));
                cpu.regs.write(rd, 0);
                return None;
            },
            RV32AInstruction::AmoswapW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = cpu.regs.read(rs2);
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmoaddW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = cpu.regs.read(rs2) + t;
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmoxorW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = cpu.regs.read(rs2) ^ t;
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmoandW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = cpu.regs.read(rs2) & t;
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmoorW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = cpu.regs.read(rs2) | t;
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmominW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = if (cpu.regs.read(rs2) as i32) < (t as i32) {
                    cpu.regs.read(rs2)
                } else {
                    t
                };
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmomaxW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = if (cpu.regs.read(rs2) as i32) > (t as i32) {
                    cpu.regs.read(rs2)
                } else {
                    t
                };
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmominuW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = if cpu.regs.read(rs2) < t {
                    cpu.regs.read(rs2)
                } else {
                    t
                };
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
            RV32AInstruction::AmomaxuW(rd, rs1, rs2) => {
                let address: u32 = cpu.regs.read(rs1);
                let t: u32 = cpu.mem.read_word(address as usize);
                let data: u32 = if cpu.regs.read(rs2) > t {
                    cpu.regs.read(rs2)
                } else {
                    t
                };
                cpu.mem.write_word(address as usize, data);
                cpu.regs.write(rd, t);
                return None;
            },
        }
    }
}