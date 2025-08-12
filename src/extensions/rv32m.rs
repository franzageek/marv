use crate::cpu;
use crate::extensions::Execute;
use crate::trap;

pub enum RV32MInstruction {
    Mul(u8, u8, u8),
    Mulh(u8, u8, u8),
    Mulhsu(u8, u8, u8),
    Mulhu(u8, u8, u8),
    Div(u8, u8, u8),
    Divu(u8, u8, u8),
    Rem(u8, u8, u8),
    Remu(u8, u8, u8),
}

impl std::fmt::Debug for RV32MInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mul(rd, rs1, rs2) => write!(f, "RV32MInstruction::Mul {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Mulh(rd, rs1, rs2) => write!(f, "RV32MInstruction::Mulh {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Mulhsu(rd, rs1, rs2) => write!(f, "RV32MInstruction::Mulhsu {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Mulhu(rd, rs1, rs2) => write!(f, "RV32MInstruction::Mulhu {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Div(rd, rs1, rs2) => write!(f, "RV32MInstruction::Div {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Divu(rd, rs1, rs2) => write!(f, "RV32MInstruction::Divu {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Rem(rd, rs1, rs2) => write!(f, "RV32MInstruction::Rem {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Remu(rd, rs1, rs2) => write!(f, "RV32MInstruction::Remu {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
        }
    }
}

impl Execute for RV32MInstruction {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap> {
        match self {
            RV32MInstruction::Mul(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1).wrapping_mul(cpu.regs.read(rs2));
                cpu.regs.write(rd, data);
                return None;
            },
            RV32MInstruction::Mulh(rd, rs1, rs2) => {
                let data: i64 = (cpu.regs.read(rs1) as i32 * cpu.regs.read(rs2) as i32) as i64;
                cpu.regs.write(rd, (data >> 32) as u32);
                return None;
            },
            RV32MInstruction::Mulhsu(rd, rs1, rs2) => {
                let data: i64 = cpu.regs.read(rs1) as i64 * cpu.regs.read(rs2) as u64 as i64;
                cpu.regs.write(rd, (data >> 32) as u32);
                return None;
            },
            RV32MInstruction::Mulhu(rd, rs1, rs2) => {
                let data: u64 = cpu.regs.read(rs1) as u64 * cpu.regs.read(rs2) as u64;
                cpu.regs.write(rd, (data >> 32) as u32);
                return None;
            },
            RV32MInstruction::Div(rd, rs1, rs2) => {
                if cpu.regs.read(rs2) == 0 {
                    panic!("Division by zero");
                }
                let data: i32 = cpu.regs.read(rs1) as i32 / cpu.regs.read(rs2) as i32;
                cpu.regs.write(rd, data as u32);
                return None;
            },
            RV32MInstruction::Divu(rd, rs1, rs2) => {
                if cpu.regs.read(rs2) == 0 {
                    panic!("Division by zero");
                }
                let data: u32 = cpu.regs.read(rs1) / cpu.regs.read(rs2);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32MInstruction::Rem(rd, rs1, rs2) => {
                if cpu.regs.read(rs2) == 0 {
                    panic!("Division by zero");
                }
                let data: i32 = cpu.regs.read(rs1) as i32 % cpu.regs.read(rs2) as i32;
                cpu.regs.write(rd, data as u32);
                return None;
            },
            RV32MInstruction::Remu(rd, rs1, rs2) => {
                if cpu.regs.read(rs2) == 0 {
                    panic!("Division by zero");
                }
                let data: u32 = cpu.regs.read(rs1) % cpu.regs.read(rs2);
                cpu.regs.write(rd, data);
                return None;
            },
        }
    }
}