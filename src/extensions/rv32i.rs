use crate::{cpu, uart};
use crate::extensions::Execute;
use crate::trap;

pub enum RV32IInstruction {
    Lui(u8, i32),
    Auipc(u8, i32),
    Jal(u8, i32),
    Jalr(u8, u8, i32),
    Beq(u8, u8, i32),
    Bne(u8, u8, i32),
    Blt(u8, u8, i32),
    Bge(u8, u8, i32),
    Bltu(u8, u8, i32),
    Bgeu(u8, u8, i32),
    Lb(u8, u8, i32),
    Lh(u8, u8, i32),
    Lw(u8, u8, i32),
    Lbu(u8, u8, i32),
    Lhu(u8, u8, i32),
    Sb(u8, u8, i32),
    Sh(u8, u8, i32),
    Sw(u8, u8, i32),
    Addi(u8, u8, i32),
    Slti(u8, u8, i32),
    Sltiu(u8, u8, i32),
    Xori(u8, u8, i32),
    Ori(u8, u8, i32),
    Andi(u8, u8, i32),
    Slli(u8, u8, u8),
    Srli(u8, u8, u8),
    Srai(u8, u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Sll(u8, u8, u8),
    Slt(u8, u8, u8),
    Sltu(u8, u8, u8),
    Xor(u8, u8, u8),
    Srl(u8, u8, u8),
    Sra(u8, u8, u8),
    Or(u8, u8, u8),
    And(u8, u8, u8),
    Fence(u8, u8, u8, u8, u8),
    FenceTSO,
    Pause,
    Ecall,
    Ebreak,
}

impl std::fmt::Debug for RV32IInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //Self::Unknown => write!(f, "Instruction::Unknown"),
            Self::Lui(rd, imm) => write!(f, "Instruction::Lui {{ rd:x{rd}, imm:{imm} }}"),
            Self::Auipc(rd, imm) => write!(f, "Instruction::Auipc {{ rd:x{rd}, imm:{imm} }}"),
            Self::Jal(rd, imm) => write!(f, "Instruction::Jal {{ rd:x{rd}, imm:{imm} }}"),
            Self::Jalr(rd, rs1, imm) => write!(f, "Instruction::Jalr {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Beq(rs1, rs2, imm) => write!(f, "Instruction::Beq {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Bne(rs1, rs2, imm) => write!(f, "Instruction::Bne {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Blt(rs1, rs2, imm) => write!(f, "Instruction::Blt {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Bge(rs1, rs2, imm) => write!(f, "Instruction::Bge {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Bltu(rs1, rs2, imm) => write!(f, "Instruction::Bltu {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Bgeu(rs1, rs2, imm) => write!(f, "Instruction::Bgeu {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Lb(rd, rs1, imm) => write!(f, "Instruction::Lb {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Lh(rd, rs1, imm) => write!(f, "Instruction::Lh {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Lw(rd, rs1, imm) => write!(f, "Instruction::Lw {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Lbu(rd, rs1, imm) => write!(f, "Instruction::Lbu {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Lhu(rd, rs1, imm) => write!(f, "Instruction::Lhu {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Sb(rs1, rs2, imm) => write!(f, "Instruction::Sb {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Sh(rs1, rs2, imm) => write!(f, "Instruction::Sh {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Sw(rs1, rs2, imm) => write!(f, "Instruction::Sw {{ rs1:x{rs1}, rs2:x{rs2}, imm:{imm} }}"),
            Self::Addi(rd, rs1, imm) => write!(f, "Instruction::Addi {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Slti(rd, rs1, imm) => write!(f, "Instruction::Slti {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Sltiu(rd, rs1, imm) => write!(f, "Instruction::Sltiu {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Xori(rd, rs1, imm) => write!(f, "Instruction::Xori {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Ori(rd, rs1, imm) => write!(f, "Instruction::Ori {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Andi(rd, rs1, imm) => write!(f, "Instruction::Andi {{ rd:x{rd}, rs1:x{rs1}, imm:{imm} }}"),
            Self::Slli(rd, rs1, shamt) => write!(f, "Instruction::Slli {{ rd:x{rd}, rs1:x{rs1}, shamt:{shamt} }}"),
            Self::Srli(rd, rs1, shamt) => write!(f, "Instruction::Srli {{ rd:x{rd}, rs1:x{rs1}, shamt:{shamt} }}"),
            Self::Srai(rd, rs1, shamt) => write!(f, "Instruction::Srai {{ rd:x{rd}, rs1:x{rs1}, shamt:{shamt} }}"),
            Self::Add(rd, rs1, rs2) => write!(f, "Instruction::Add {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Sub(rd, rs1, rs2) => write!(f, "Instruction::Sub {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Sll(rd, rs1, rs2) => write!(f, "Instruction::Sll {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Slt(rd, rs1, rs2) => write!(f, "Instruction::Slt {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Sltu(rd, rs1, rs2) => write!(f, "Instruction::Sltu {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Xor(rd, rs1, rs2) => write!(f, "Instruction::Xor {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Srl(rd, rs1, rs2) => write!(f, "Instruction::Srl {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Sra(rd, rs1, rs2) => write!(f, "Instruction::Sra {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Or(rd, rs1, rs2) => write!(f, "Instruction::Or {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::And(rd, rs1, rs2) => write!(f, "Instruction::And {{ rd:x{rd}, rs1:x{rs1}, rs2:x{rs2} }}"),
            Self::Fence(rd, rs1, succ, pred, fm) => write!(f, "Instruction::Fence x{}, x{}, {}, {}, {}", rd, rs1, succ, pred, fm),
            Self::FenceTSO => write!(f, "Instruction::Fence.TSO"),
            Self::Pause => write!(f, "Instruction::Pause"),
            Self::Ecall => write!(f, "Instruction::Ecall"),
            Self::Ebreak => write!(f, "Instruction::Ebreak"),
        }
    }
}


impl Execute for RV32IInstruction {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap> {
        match self {
            RV32IInstruction::Lui(rd, imm) => {
                cpu.regs.write(rd, imm as u32);
                return None;
            },
            RV32IInstruction::Auipc(rd, imm) => {
                cpu.regs.write(rd, cpu.regs.pc.wrapping_add_signed(imm-4)); // PC is updated right after - sure with branch instructions, further investigation required for loads
                return None;
            },
            RV32IInstruction::Jal(rd, imm) => {
                cpu.regs.write(
                    if rd == 0 {
                        1
                    } else {
                        rd
                    },
                    cpu.regs.pc.wrapping_add(4) // PC is updated right after - sure with branch instructions, further investigation required for jumps
                );
                cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                if cpu.regs.pc & 0x3 != 0 {
                    return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                }
                return None;
            },
            RV32IInstruction::Jalr(rd, rs1, imm) => {
                let t: u32 = cpu.regs.pc.wrapping_add(4);
                cpu.regs.pc = cpu.regs.read(rs1).wrapping_add_signed(imm-4);
                if cpu.regs.pc & 0x3 != 0 {
                    return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                }
                cpu.regs.write(
                    if rd == 0 {
                        1
                    } else {
                        rd
                    },
                    t
                );
                //println!("{:08X} {}", cpu.regs.x[rs1 as usize], imm);
                return None;
            },
            RV32IInstruction::Beq(rs1, rs2, imm) => {
                if cpu.regs.read(rs1) == cpu.regs.read(rs2) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4); // cause PC gets updated right after
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Bne(rs1, rs2, imm) => {
                if cpu.regs.read(rs1) != cpu.regs.read(rs2) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Blt(rs1, rs2, imm) => {
                if (cpu.regs.read(rs1) as i32) < (cpu.regs.read(rs2) as i32) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Bge(rs1, rs2, imm) => {
                if (cpu.regs.read(rs1) as i32) >= (cpu.regs.read(rs2) as i32) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Bltu(rs1, rs2, imm) => {
                if cpu.regs.read(rs1) < cpu.regs.read(rs2) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Bgeu(rs1, rs2, imm) => {
                if cpu.regs.read(rs1) >= cpu.regs.read(rs2) {
                    cpu.regs.pc = cpu.regs.pc.wrapping_add_signed(imm-4);
                    if cpu.regs.pc & 0x3 != 0 {
                        return Some(trap::Trap::take(trap::Trap::MisalignedInstructionAddress, cpu, cpu.regs.pc));
                    }
                }
                return None;
            },
            RV32IInstruction::Lb(rd, rs1, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                let mut ubyte: u8 = cpu.mem.read_byte(address as usize);
                if uart::match_addr(address) {
                    if let Some(data) = cpu.uart.read(address) {
                        ubyte = data;
                    }
                }
                let idata: i32 = ((ubyte as i32) << 24) >> 24;
                cpu.regs.write(rd, idata as u32);
                return None;
            },
            RV32IInstruction::Lh(rd, rs1, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                if uart::match_addr(address) {
                    return Some(trap::Trap::take(trap::Trap::LoadAccessFault, cpu, cpu.regs.pc));
                }
                let uhalf: u16 = cpu.mem.read_half_word(address as usize);
                let idata: i32 = ((uhalf as i32) << 16) >> 16;
                cpu.regs.write(rd, idata as u32);
                return None;
            },
            RV32IInstruction::Lw(rd, rs1, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                if uart::match_addr(address) {
                    return Some(trap::Trap::take(trap::Trap::LoadAccessFault, cpu, cpu.regs.pc));
                }
                let udata: u32 = cpu.mem.read_word(address as usize);
                cpu.regs.write(rd, udata);
                return None;
            },
            RV32IInstruction::Lbu(rd, rs1, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                let mut ubyte: u8 = cpu.mem.read_byte(address as usize);
                if uart::match_addr(address) {
                    if let Some(data) = cpu.uart.read(address) {
                        ubyte = data;
                    }
                }
                cpu.regs.write(rd, (ubyte as u32) & !0xFF);
                return None;
            },
            RV32IInstruction::Lhu(rd, rs1, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                if uart::match_addr(address) {
                    return Some(trap::Trap::take(trap::Trap::LoadAccessFault, cpu, cpu.regs.pc));
                }
                let uhalf: u16 = cpu.mem.read_half_word(address as usize);
                cpu.regs.write(rd, (uhalf as u32) & !0xFFFF);
                return None;
            },
            RV32IInstruction::Sb(rs1, rs2, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                let byte: u8 = (cpu.regs.read(rs2) & 0xFF) as u8;
                if uart::match_addr(address) {
                    cpu.uart.write(address, byte);
                } else {
                    cpu.mem.write_byte(address as usize, byte);
                }
                return None;
            },
            RV32IInstruction::Sh(rs1, rs2, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                if uart::match_addr(address) {
                    return Some(trap::Trap::take(trap::Trap::StoreAccessFault, cpu, cpu.regs.pc));
                }
                let half: u16 = (cpu.regs.read(rs2) & 0xFFFF) as u16;
                cpu.mem.write_half_word(address as usize, half);
                return None;
            },
            RV32IInstruction::Sw(rs1, rs2, imm) => {
                let address: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                if uart::match_addr(address) {
                    return Some(trap::Trap::take(trap::Trap::StoreAccessFault, cpu, cpu.regs.pc));
                }
                cpu.mem.write_word(address as usize, cpu.regs.read(rs2));
                return None;
            },
            RV32IInstruction::Addi(rd, rs1, imm) => {
                let data: u32 = cpu.regs.read(rs1).wrapping_add_signed(imm);
                cpu.regs.write(rd, data);
                //println!("{rd} -> {}", cpu.regs.x[12]);
                return None;
            },
            RV32IInstruction::Slti(rd, rs1, imm) => {
                if (cpu.regs.read(rs1) as i32) < imm {
                    cpu.regs.write(rd, 1);
                } else {
                    cpu.regs.write(rd, 0);
                }
                return None;
            },
            RV32IInstruction::Sltiu(rd, rs1, imm) => {
                if cpu.regs.read(rs1) < imm as u32 {
                    cpu.regs.write(rd, 1);
                } else {
                    cpu.regs.write(rd, 0);
                }
                return None;
            },
            RV32IInstruction::Xori(rd, rs1, imm) => {
                let data: u32 = cpu.regs.read(rs1) ^ (imm as u32);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Ori(rd, rs1, imm) => {
                let data: u32 = cpu.regs.read(rs1) | (imm as u32);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Andi(rd, rs1, imm) => {
                let data: u32 = cpu.regs.read(rs1) & (imm as u32);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Slli(rd, rs1, shamt) => {
                let data: u32 = cpu.regs.read(rs1) << shamt;
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Srli(rd, rs1, shamt) => {
                let data: u32 = cpu.regs.read(rs1) >> shamt;
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Srai(/*rd, rs1, shamt*/_, _, _) => {
                /*
                let data: u32 = ((cpu.regs.read(rs1) as i32) >> shamt) as u32;
                cpu.regs.write(rd, data);
                */
                //panic!("Illegal instruction: SRAI");
                return Some(trap::Trap::take(trap::Trap::IllegalInstruction, cpu, cpu.regs.pc));
            },
            RV32IInstruction::Add(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1).wrapping_add(cpu.regs.read(rs2));
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Sub(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1).wrapping_sub(cpu.regs.read(rs2));
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Sll(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1) << cpu.regs.read(rs2) & 0x1F;
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Slt(rd, rs1, rs2) => {
                if (cpu.regs.read(rs1) as i32) < (cpu.regs.read(rs2) as i32) {
                    cpu.regs.write(rd, 1);
                } else {
                    cpu.regs.write(rd, 0);
                }
                return None;
            },
            RV32IInstruction::Sltu(rd, rs1, rs2) => {
                if cpu.regs.read(rs1) < cpu.regs.read(rs2) {
                    cpu.regs.write(rd, 1);
                } else {
                    cpu.regs.write(rd, 0);
                }
                return None;
            },
            RV32IInstruction::Xor(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1) ^ cpu.regs.read(rs2);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Srl(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1) >> cpu.regs.read(rs2) & 0x1F;
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Sra(rd, rs1, rs2) => {
                let data: u32 = ((cpu.regs.read(rs1) as i32) >> cpu.regs.read(rs2) & 0x1F) as u32;
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Or(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1) | cpu.regs.read(rs2);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::And(rd, rs1, rs2) => {
                let data: u32 = cpu.regs.read(rs1) & cpu.regs.read(rs2);
                cpu.regs.write(rd, data);
                return None;
            },
            RV32IInstruction::Fence(_, _, _, _, _) => return None,
            RV32IInstruction::FenceTSO => return None,
            RV32IInstruction::Pause => return None,
            RV32IInstruction::Ecall => {
                match cpu.privilege {
                    0 => return Some(trap::Trap::take(trap::Trap::UModeEnvCall, cpu, cpu.regs.pc)),
                    1 => return Some(trap::Trap::take(trap::Trap::SModeEnvCall, cpu, cpu.regs.pc)),
                    3 => return Some(trap::Trap::take(trap::Trap::MModeEnvCall, cpu, cpu.regs.pc)),
                    _ => return Some(trap::Trap::take(trap::Trap::IllegalInstruction, cpu, cpu.regs.pc)),
                }
            },
            RV32IInstruction::Ebreak => {
                return Some(trap::Trap::take(trap::Trap::Breakpoint, cpu, cpu.regs.pc));
            },
            //_ => panic!("Unimplmemented instruction"),
        }
    }
}
