use crate::instruction::*;
use crate::trap::*;

pub fn rv32_decode(instr: u32) -> RV32Instruction {
    let opcode: u8 = (instr & 0x7F) as u8;
    if let Some(t) = &OPTABLE[opcode as usize] {
        match t {
            Type::R => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let funct3: u8 = ((instr >> (7 + 5)) & 0x7) as u8;
                let rs1: u8 = ((instr >> 7 + 5 + 3) & 0x1F) as u8;
                let rs2: u8 = ((instr >> 7 + 5 + 3 + 5) & 0x1F) as u8;
                let funct7: u8 = ((instr >> 7 + 5 + 3 + 5 + 5) & 0x7F) as u8;
                match opcode {
                    0b0110011 => match funct3 {
                        0b000 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Add(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Mul(rd, rs1, rs2)),
                            0b0100000 => return RV32Instruction::RV32I(RV32IInstruction::Sub(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b001 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Sll(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Mulh(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b010 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Slt(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Mulhsu(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b011 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Sltu(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Mulhu(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b100 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Xor(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Div(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b101 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Srl(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Divu(rd, rs1, rs2)),
                            0b0100000 => return RV32Instruction::RV32I(RV32IInstruction::Sra(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b110 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Or(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Rem(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b111 => match funct7 {
                            0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::And(rd, rs1, rs2)),
                            0b0000001 => return RV32Instruction::RV32M(RV32MInstruction::Remu(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32Instruction::Unknown;
                            },
                        },
                        _ => {
                            eprintln!("Unknown R-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    0b0101111 => match funct3 {
                        0b010 => match (funct7 & !0x3) >> 2 {
                            0b00000 => return RV32Instruction::RV32A(RV32AInstruction::AmoaddW(rd, rs1, rs2)),
                            0b00001 => return RV32Instruction::RV32A(RV32AInstruction::AmoswapW(rd, rs1, rs2)),
                            0b00010 => return RV32Instruction::RV32I(RV32IInstruction::Lw(rd, rs1, 0)), // TEMPORARY TRICK: replace with actual instruction
                            0b00011 => return RV32Instruction::RV32A(RV32AInstruction::ScW(rd, rs1, rs2)),
                            0b00100 => return RV32Instruction::RV32A(RV32AInstruction::AmoxorW(rd, rs1, rs2)),
                            0b01100 => return RV32Instruction::RV32A(RV32AInstruction::AmoandW(rd, rs1, rs2)),
                            0b01000 => return RV32Instruction::RV32A(RV32AInstruction::AmoorW(rd, rs1, rs2)),
                            0b10000 => return RV32Instruction::RV32A(RV32AInstruction::AmominW(rd, rs1, rs2)),
                            0b10100 => return RV32Instruction::RV32A(RV32AInstruction::AmomaxW(rd, rs1, rs2)),
                            0b11000 => return RV32Instruction::RV32A(RV32AInstruction::AmominuW(rd, rs1, rs2)),
                            0b11100 => return RV32Instruction::RV32A(RV32AInstruction::AmomaxuW(rd, rs1, rs2)),
                            _ => {
                                eprintln!("Unknown R-type A instruction with funct5: 0b{:05b}", (funct7 & !0x3) >> 2);
                                return RV32Instruction::Unknown;
                            },
                        },
                        _ => {
                            eprintln!("Unknown R-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown R-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
            Type::I => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let funct3: u8 = ((instr >> (7 + 5)) & 0x7) as u8;
                let rs1: u8 = ((instr >> 7 + 5 + 3) & 0x1F) as u8;
                let uimm: u32 = ((instr >> 7 + 5 + 3 + 5) & 0xFFF) as u32;
                let iimm: i32 = ((uimm as i32) << 20) >> 20;
                match opcode {
                    0b0000011 => match funct3 {
                        0b000 => return RV32Instruction::RV32I(RV32IInstruction::Lb(rd, rs1, iimm)),
                        0b001 => return RV32Instruction::RV32I(RV32IInstruction::Lh(rd, rs1, iimm)),
                        0b010 => return RV32Instruction::RV32I(RV32IInstruction::Lw(rd, rs1, iimm)),
                        0b100 => return RV32Instruction::RV32I(RV32IInstruction::Lbu(rd, rs1, iimm)),
                        0b101 => return RV32Instruction::RV32I(RV32IInstruction::Lhu(rd, rs1, iimm)),
                        _ => {
                            eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    0b0001111 => match uimm {
                        0b100000110011 => return RV32Instruction::RV32I(RV32IInstruction::FenceTSO),
                        0b000000010000 => return RV32Instruction::RV32I(RV32IInstruction::Pause),
                        _ => {
                            let succ: u8 = (uimm & 0xF) as u8;
                            let pred: u8 = ((uimm >> 4) & 0xF) as u8;
                            let fm: u8 = ((uimm >> 8) & 0xF) as u8;
                            return RV32Instruction::RV32I(RV32IInstruction::Fence(rd, rs1, succ, pred, fm));
                        },
                    },
                    0b0010011 => match funct3 {
                        0b000 => return RV32Instruction::RV32I(RV32IInstruction::Addi(rd, rs1, iimm)),
                        0b001 => {
                            let shamt: u8 = (uimm & 0x1F) as u8;
                            return RV32Instruction::RV32I(RV32IInstruction::Slli(rd, rs1, shamt));
                        },
                        0b010 => return RV32Instruction::RV32I(RV32IInstruction::Slti(rd, rs1, iimm)),
                        0b011 => return RV32Instruction::RV32I(RV32IInstruction::Sltiu(rd, rs1, iimm)),
                        0b100 => return RV32Instruction::RV32I(RV32IInstruction::Xori(rd, rs1, iimm)),
                        0b101 => {
                            let shamt: u8 = (uimm & 0x1F) as u8;
                            match uimm >> 5 {
                                0b0000000 => return RV32Instruction::RV32I(RV32IInstruction::Srli(rd, rs1, shamt)),
                                0b0100000 => return RV32Instruction::RV32I(RV32IInstruction::Srai(rd, rs1, shamt)),
                                _ => {
                                    eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                                    return RV32Instruction::Unknown;
                                },
                            }
                        },
                        0b110 => return RV32Instruction::RV32I(RV32IInstruction::Ori(rd, rs1, iimm)),
                        0b111 => return RV32Instruction::RV32I(RV32IInstruction::Andi(rd, rs1, iimm)),
                        _ => {
                            eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    0b1100111 => return RV32Instruction::RV32I(RV32IInstruction::Jalr(rd, rs1, iimm)),
                    0b1110011 => match funct3 {
                        0b000 => match uimm {
                            0b000000000000 => return RV32Instruction::RV32I(RV32IInstruction::Ecall),
                            0b000000000001 => return RV32Instruction::RV32I(RV32IInstruction::Ebreak),
                            0b000100000010 => return RV32Instruction::TrapReturn(TrapRetInstruction::Sret),
                            0b001100000010 => return RV32Instruction::TrapReturn(TrapRetInstruction::Mret),
                            _ => {
                                eprintln!("Unknown I-type instruction with imm[11:0]: 0b{:012b}", iimm);
                                return RV32Instruction::Unknown;
                            },
                        },
                        0b001 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrw(rd, rs1, (uimm & 0xFFF) as u16)),
                        0b010 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrs(rd, rs1, (uimm & 0xFFF) as u16)),
                        0b011 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrc(rd, rs1, (uimm & 0xFFF) as u16)),
                        0b101 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrwi(rd, rs1, (uimm & 0xFFF) as u16)),
                        0b110 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrsi(rd, rs1, (uimm & 0xFFF) as u16)),
                        0b111 => return RV32Instruction::RV32Ziscr(RV32ZicsrInstruction::Csrrci(rd, rs1, (uimm & 0xFFF) as u16)),
                        _ => {
                            eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown I-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
            Type::S => {
                let imm0: u8 = ((instr >> 7) & 0x1F) as u8;
                let funct3: u8 = ((instr >> (7 + 5)) & 0x7) as u8;
                let rs1: u8 = ((instr >> 7 + 5 + 3) & 0x1F) as u8;
                let rs2: u8 = ((instr >> 7 + 5 + 3 + 5) & 0x1F) as u8;
                let imm1: u8 = ((instr >> 7 + 5 + 3 + 5 + 5) & 0x7F) as u8;
                let uimm: u32 = ((imm1 << 5) | imm0) as u32;
                let iimm: i32 = ((uimm as i32) << 20) >> 20;
                match opcode {
                    0b0100011 => match funct3 {
                        0b000 => return RV32Instruction::RV32I(RV32IInstruction::Sb(rs2, rs1, iimm)),
                        0b001 => return RV32Instruction::RV32I(RV32IInstruction::Sh(rs2, rs1, iimm)),
                        0b010 => return RV32Instruction::RV32I(RV32IInstruction::Sw(rs2, rs1, iimm)),
                        _ => {
                            eprintln!("Unknown S-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown S-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
            Type::B => {
                let imm0: u8 = ((instr >> 7) & 0x1F) as u8;
                let funct3: u8 = ((instr >> (7 + 5)) & 0x7) as u8;
                let rs1: u8 = ((instr >> 7 + 5 + 3) & 0x1F) as u8;
                let rs2: u8 = ((instr >> 7 + 5 + 3 + 5) & 0x1F) as u8;
                let imm1: u8 = ((instr >> 7 + 5 + 3 + 5 + 5) & 0x7F) as u8;
                let uimm: u32 = ((((imm1 as u16 >> 6) & 0x1) << 12) | ((imm0 as u16 & 0x1) << 11) | ((imm1 as u16 & 0x3F) << 5) | (imm0 as u16 & 0x1E)) as u32;
                let iimm: i32 = ((uimm as i32) << 19) >> 19;
                match opcode {
                    0b1100011 => match funct3 {
                        0b000 => return RV32Instruction::RV32I(RV32IInstruction::Beq(rs1, rs2, iimm)),
                        0b001 => return RV32Instruction::RV32I(RV32IInstruction::Bne(rs1, rs2, iimm)),
                        0b100 => return RV32Instruction::RV32I(RV32IInstruction::Blt(rs1, rs2, iimm)),
                        0b101 => return RV32Instruction::RV32I(RV32IInstruction::Bge(rs1, rs2, iimm)),
                        0b110 => return RV32Instruction::RV32I(RV32IInstruction::Bltu(rs1, rs2, iimm)),
                        0b111 => return RV32Instruction::RV32I(RV32IInstruction::Bgeu(rs1, rs2, iimm)),
                        _ => {
                            eprintln!("Unknown B-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32Instruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown B-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
            Type::U => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let uimm: u32 = (instr & 0xFFFFF000) as u32;
                let iimm: i32 = uimm as i32;
                match opcode {
                    0b0110111 => return RV32Instruction::RV32I(RV32IInstruction::Lui(rd, iimm)),
                    0b0010111 => return RV32Instruction::RV32I(RV32IInstruction::Auipc(rd, iimm)),
                    _ => {
                        eprintln!("Unknown U-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
            Type::J => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let imm: u32 = ((instr >> 7 + 5) & 0xFFFFF) as u32;
                let segment0: u8 = (imm & 0xFF) as u8;
                let segment1: u8 = ((imm >> 8) & 0x1) as u8;
                let segment2: u16 = ((imm >> 9) & 0x3FF) as u16;
                let segment3: u8 = ((imm >> 19) & 0x1) as u8;
                let uimm: u32 = (((segment3 as u32) << 20) as u32 | ((segment0 as u32) << 12) as u32 | ((segment1 as u32) << 11) as u32 | (segment2 << 1) as u32) as u32;
                let iimm: i32 = ((uimm as i32) << 11) >> 11;
                match opcode {
                    0b1101111 => return RV32Instruction::RV32I(RV32IInstruction::Jal(rd, iimm)),
                    _ => {
                        eprintln!("Unknown J-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32Instruction::Unknown;
                    },
                }
            },
        }
    } else {
        return RV32Instruction::Unknown;
    }
}
