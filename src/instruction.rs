pub enum RV32IInstruction {
    Unknown,
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
            Self::Unknown => write!(f, "Instruction::Unknown"),
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

pub enum Type {
    R,
    I,
    S,
    B,
    U,
    J,
}

const OPTABLE: [Option<Type>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(Type::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(Type::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(Type::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(Type::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ None,
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(Type::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(Type::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(Type::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ None,
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(Type::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(Type::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ Some(Type::J),
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(Type::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];

pub fn rv32_decode(instr: u32) -> RV32IInstruction {
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
                            0b0000000 => return RV32IInstruction::Add(rd, rs1, rs2),
                            0b0100000 => return RV32IInstruction::Sub(rd, rs1, rs2),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32IInstruction::Unknown;
                            },
                        },
                        0b001 => return RV32IInstruction::Sll(rd, rs1, rs2),
                        0b010 => return RV32IInstruction::Slt(rd, rs1, rs2),
                        0b011 => return RV32IInstruction::Sltu(rd, rs1, rs2),
                        0b100 => return RV32IInstruction::Xor(rd, rs1, rs2),
                        0b101 => match funct7 {
                            0b0000000 => return RV32IInstruction::Srl(rd, rs1, rs2),
                            0b0100000 => return RV32IInstruction::Sra(rd, rs1, rs2),
                            _ => {
                                eprintln!("Unknown R-type instruction with funct7: 0b{:07b}", funct7);
                                return RV32IInstruction::Unknown;
                            },
                        },
                        0b110 => return RV32IInstruction::Or(rd, rs1, rs2),
                        0b111 => return RV32IInstruction::And(rd, rs1, rs2),
                        _ => {
                            eprintln!("Unknown R-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown R-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
                    },
                }
            },
            Type::I => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let funct3: u8 = ((instr >> (7 + 5)) & 0x7) as u8;
                let rs1: u8 = ((instr >> 7 + 5 + 3) & 0x1F) as u8;
                let uimm: u32 = ((instr >> 7 + 5 + 3 + 5) & 0x7FF) as u32;
                let iimm: i32 = ((uimm as i32) << 20) >> 20;
                match opcode {
                    0b0000011 => match funct3 {
                        0b000 => return RV32IInstruction::Lb(rd, rs1, iimm),
                        0b001 => return RV32IInstruction::Lh(rd, rs1, iimm),
                        0b010 => return RV32IInstruction::Lw(rd, rs1, iimm),
                        0b100 => return RV32IInstruction::Lbu(rd, rs1, iimm),
                        0b101 => return RV32IInstruction::Lhu(rd, rs1, iimm),
                        _ => {
                            eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    0b0001111 => match uimm {
                        0b100000110011 => return RV32IInstruction::FenceTSO,
                        0b000000010000 => return RV32IInstruction::Pause,
                        _ => {
                            let succ: u8 = (uimm & 0xF) as u8;
                            let pred: u8 = ((uimm >> 4) & 0xF) as u8;
                            let fm: u8 = ((uimm >> 8) & 0xF) as u8;
                            return RV32IInstruction::Fence(rd, rs1, succ, pred, fm);
                        },
                    },
                    0b0010011 => match funct3 {
                        0b000 => return RV32IInstruction::Addi(rd, rs1, iimm),
                        0b001 => {
                            let shamt: u8 = (uimm & 0x1F) as u8;
                            return RV32IInstruction::Slli(rd, rs1, shamt);
                        },
                        0b010 => return RV32IInstruction::Slti(rd, rs1, iimm),
                        0b011 => return RV32IInstruction::Sltiu(rd, rs1, iimm),
                        0b100 => return RV32IInstruction::Xori(rd, rs1, iimm),
                        0b101 => {
                            let shamt: u8 = (uimm & 0x1F) as u8;
                            match uimm >> 5 {
                                0b0000000 => return RV32IInstruction::Srli(rd, rs1, shamt),
                                0b0100000 => return RV32IInstruction::Srai(rd, rs1, shamt),
                                _ => {
                                    eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                                    return RV32IInstruction::Unknown;
                                },
                            }
                        },
                        0b110 => return RV32IInstruction::Ori(rd, rs1, iimm),
                        0b111 => return RV32IInstruction::Andi(rd, rs1, iimm),
                        _ => {
                            eprintln!("Unknown I-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    0b1100111 => return RV32IInstruction::Jalr(rd, rs1, iimm),
                    0b1110011 => match uimm {
                        0b000000000000 => return RV32IInstruction::Ecall,
                        0b000000000001 => return RV32IInstruction::Ebreak,
                        _ => {
                            eprintln!("Unknown I-type instruction with imm[11:0]: 0b{:012b}", iimm);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown I-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
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
                        0b000 => return RV32IInstruction::Sb(rs2, rs1, iimm),
                        0b001 => return RV32IInstruction::Sh(rs2, rs1, iimm),
                        0b010 => return RV32IInstruction::Sw(rs2, rs1, iimm),
                        _ => {
                            eprintln!("Unknown S-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown S-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
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
                        0b000 => return RV32IInstruction::Beq(rs1, rs2, iimm),
                        0b001 => return RV32IInstruction::Bne(rs1, rs2, iimm),
                        0b100 => return RV32IInstruction::Blt(rs1, rs2, iimm),
                        0b101 => return RV32IInstruction::Bge(rs1, rs2, iimm),
                        0b110 => return RV32IInstruction::Bltu(rs1, rs2, iimm),
                        0b111 => return RV32IInstruction::Bgeu(rs1, rs2, iimm),
                        _ => {
                            eprintln!("Unknown B-type instruction with funct3: 0b{:03b}", funct3);
                            return RV32IInstruction::Unknown;
                        },
                    },
                    _ => {
                        eprintln!("Unknown B-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
                    },
                }
            },
            Type::U => {
                let rd: u8 = ((instr >> 7) & 0x1F) as u8;
                let uimm: u32 = (instr & 0xFFFFF000) as u32;
                let iimm: i32 = uimm as i32;
                match opcode {
                    0b0110111 => return RV32IInstruction::Lui(rd, iimm),
                    0b0010111 => return RV32IInstruction::Auipc(rd, iimm),
                    _ => {
                        eprintln!("Unknown U-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
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
                    0b1101111 => return RV32IInstruction::Jal(rd, iimm),
                    _ => {
                        eprintln!("Unknown J-type instruction with opcode: 0b{:07b}", opcode);
                        return RV32IInstruction::Unknown;
                    },
                }
            },
        }
    } else {
        return RV32IInstruction::Unknown;
    }
}
