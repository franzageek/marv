use crate::decode;
use crate::trap;
use crate::instruction::*;
use crate::trap::TrapRetInstruction;
use colored::Colorize;
use std::io::Write;

#[allow(dead_code)]
pub struct RV32CSRs {
    pub mstatus: u32,
    pub misa: u32,
    pub medeleg: u32,
    pub mideleg: u32,
    pub mie: u32,
    pub mtvec: u32,
    pub mcounteren: u32,
    pub mscratch: u32,
    pub mepc: u32,
    pub mcause: u32,
    pub mtval: u32,
    pub mip: u32,
    pub mhartid: u32,
    pub mvendorid: u32,
    pub marchid: u32,
    pub mimpid: u32,

    pub sstatus: u32,
    pub sie: u32,
    pub stvec: u32,
    pub sscratch: u32,
    pub sepc: u32,
    pub scause: u32,
    pub stval: u32,
    pub sip: u32,
    pub satp: u32,

    pub cycle: u32,
    pub time: u32,
    pub instret: u32,
    pub cycleh: u32,
    pub timeh: u32,
    pub instreth: u32,
}

#[allow(dead_code)]
pub struct RV32Regs {
    pub x: [u32; 32],
    pub pc: u32,
    pub csr: RV32CSRs,
}

pub struct RV32Memory {
    pub ram: Vec<u8>,
}

pub struct RiscV32 {
    pub regs: RV32Regs,
    pub mem: RV32Memory,
    pub privilege: u8, // 0 = user, 1 = supervisor, 3 = machine
    pub status: bool
}

impl RV32Regs {
    pub fn new() -> RV32Regs {
        print!("initializing X registers...");
        let x: [u32; 32] = [0u32; 32];
        println!("{}, allocated {} X registers, {} bytes each, {} bytes total", "done".green(), x.len().to_string().blue(), size_of::<u32>().to_string().blue(), (x.len() * size_of::<u32>()).to_string().blue());
        return RV32Regs {
            x: [0u32; 32],
            pc: 0,
            csr: RV32CSRs {
                mstatus: 0,
                misa: 0,
                medeleg: 0,
                mideleg: 0,
                mie: 0,
                mtvec: 0,
                mcounteren: 0,
                mscratch: 0,
                mepc: 0,
                mcause: 0,
                mtval: 0,
                mip: 0,
                mhartid: 0,

                sstatus: 0,
                sie: 0,
                stvec: 0,
                sscratch: 0,
                sepc: 0,
                scause: 0,
                stval: 0,
                sip: 0,

                cycle: 0,
                time: 0,
                instret: 0,
                cycleh: 0,
                timeh: 0,
                instreth: 0,

                mvendorid: 0x00000000, // Vendor ID
                marchid: 0x00000000, // Architecture ID
                mimpid: 0x00000000, // Implementation ID
                satp: 0x00000000, // Supervisor Address Translation and Protection
            }
        };
    }
    pub fn read(&mut self, reg: u8) -> u32 {
        return if reg > 0 && reg < 32 {
            self.x[reg as usize]
        } else {
            0
        };
    }
    pub fn write(&mut self, reg: u8, data: u32) {
        if reg > 0 && reg < 32 {
            self.x[reg as usize] = data;
        }
        return;
    }
}

impl RV32Memory {
    pub fn new() -> RV32Memory {
        print!("initializing memory...");
        let ram: Vec<u8> = vec![0u8; (1u64 << 32) as usize];
        println!("{}, allocated {} bytes", "done".green(), ram.len().to_string().blue());
        return RV32Memory {
            ram: ram,
        };
    }

    pub fn read_byte(&mut self, address: usize) -> u8 {
        return self.ram[address];
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) { // maybe use the equivalent of memcpy for write ops
        self.ram[address] = byte;
        return;
    }

    pub fn read_half_word(&mut self, address: usize) -> u16 {
        return u16::from_le_bytes([self.ram[address], self.ram[address+1]]);
    }

    pub fn write_half_word(&mut self, address: usize, half: u16) {
        self.ram[address] = (half & 0xFF) as u8;
        self.ram[address+1] = ((half >> 8) & 0xFF) as u8;
        return;
    }

    pub fn read_word(&mut self, address: usize) -> u32 {
        return u32::from_le_bytes([self.ram[address], self.ram[address+1], self.ram[address+2], self.ram[address+3]]);
    }

    pub fn write_word(&mut self, address: usize, word: u32) {
        self.ram[address] = (word & 0xFF) as u8;
        self.ram[address+1] = ((word >> 8) & 0xFF) as u8;
        self.ram[address+2] = ((word >> 16) & 0xFF) as u8;
        self.ram[address+3] = ((word >> 24) & 0xFF) as u8;
        return;
    }
}

#[allow(dead_code)]
impl RiscV32 {
    pub fn new() -> RiscV32 {
        return RiscV32 {
            regs: RV32Regs::new(),
            mem: RV32Memory::new(),
            privilege: 0, // user mode
            status: false
        };
    }
    pub fn reset(&mut self) {
        print!("setting processor state...");
        self.status = true;
        self.privilege = 3; // machine mode
        println!("{}", "done".green());
        print!("resetting program counter...");
        self.regs.pc = 0;
        println!("{}, execution starts at <0x{:08X}>", "done".green(), self.regs.pc);
        print!("clearing X registers...");
        self.regs.x.fill(0);
        println!("{}, all X registers have been set to 0", "done".green());
        print!("clearing RAM memory...");
        std::io::stdout().flush().unwrap();
        self.mem.ram.fill(0);
        println!("{}", "done".green());
        print!("resetting CSRs...");
        self.regs.csr.misa = (1 << 30) | (1 << 20) | (1 << 18) | (1 << 12) | (1 << 8) | (1 << 0);
        println!("{}, extensions {} + {} have been enabled, XLEN has been set to {}", "done".green(), "IMA".blue(), "SU".blue(), "32".blue());
        println!("{}", "successful RV32 processor reset".on_truecolor(0, 100, 0));
    }
    fn check_privilege(&self, csr: u16) -> bool { // [ ] give names to these constants
        if (
            self.privilege == 3 ||
            self.privilege == 1
        ) && (
            csr == 0x100 ||
            csr == 0x104 ||
            csr == 0x105 ||
            csr == 0x106 ||
            csr == 0x10A ||
            csr == 0x120 || 
            (
                csr >= 0x140 &&
                csr <= 0x144
            ) ||
            csr == 0xDA0 ||
            csr == 0x180 ||
            csr == 0x5A8 ||
            (
                csr >= 0x10C &&
                csr <= 0x10F
            )
        ) {
            return true;
        }
        if self.privilege == 3 && (
            ( 
                csr >= 0xF11 &&
                csr <= 0xF15
            ) ||
            (
                csr >= 0x300 &&
                csr <= 0x306
            ) ||
            csr == 0x310 ||
            csr == 0x312 ||
            (
                csr >= 0x340 &&
                csr <= 0x344
            ) ||
            csr == 0x34A ||
            csr == 0x34B ||
            csr == 0x30A ||
            csr == 0x31A ||
            csr == 0x747 ||
            csr == 0x757 || (
                csr >= 0x3A0 &&
                csr <= 0x3EF
            )
        ) {
            return true;
        }
        return false;
    }
    fn read_csr(&self, csr: u16) -> Result<u32, trap::Trap> {
        if self.check_privilege(csr) {
            match csr {
                0xC00 => return Ok(self.regs.csr.cycle),
                0xC01 => return Ok(self.regs.csr.time),
                0xC02 => return Ok(self.regs.csr.instret),
                0xC80 => return Ok(self.regs.csr.cycleh),
                0xC81 => return Ok(self.regs.csr.timeh),
                0xC82 => return Ok(self.regs.csr.instreth),
                0x100 => return Ok(self.regs.csr.sstatus),
                0x104 => return Ok(self.regs.csr.sie),
                0x105 => return Ok(self.regs.csr.stvec),
                //0x106 => return Ok(self.regs.csr.scounteren),
                //0x10A => return Ok(self.regs.csr.senvcfg),
                0x140 => return Ok(self.regs.csr.sscratch),
                0x141 => return Ok(self.regs.csr.sepc),
                0x142 => return Ok(self.regs.csr.scause),
                0x143 => return Ok(self.regs.csr.stval),
                0x144 => return Ok(self.regs.csr.sip),
                0x180 => return Ok(self.regs.csr.satp),
                0xF11 => return Ok(self.regs.csr.mvendorid),
                0xF12 => return Ok(self.regs.csr.marchid),
                0xF13 => return Ok(self.regs.csr.mimpid),
                0xF14 => return Ok(self.regs.csr.mhartid),
                0x300 => return Ok(self.regs.csr.mstatus),
                0x301 => return Ok(self.regs.csr.misa),
                0x302 => return Ok(self.regs.csr.medeleg),
                0x303 => return Ok(self.regs.csr.mideleg),
                0x304 => return Ok(self.regs.csr.mie),
                0x305 => return Ok(self.regs.csr.mtvec),
                0x306 => return Ok(self.regs.csr.mcounteren),
                0x340 => return Ok(self.regs.csr.sscratch),
                0x341 => return Ok(self.regs.csr.mepc),
                0x342 => return Ok(self.regs.csr.mcause),
                0x343 => return Ok(self.regs.csr.mtval),
                0x344 => return Ok(self.regs.csr.mip),
                _ => return Err(trap::Trap::IllegalInstruction),
            }
        }
        return Err(trap::Trap::IllegalInstruction);
    }
    fn write_csr(&mut self, csr: u16, data: u32) -> Option<trap::Trap> {
        if self.check_privilege(csr) {
            match csr {
                0xC00 => self.regs.csr.cycle = data,
                0xC01 => self.regs.csr.time = data,
                0xC02 => self.regs.csr.instret = data,
                0xC80 => self.regs.csr.cycleh = data,
                0xC81 => self.regs.csr.timeh = data,
                0xC82 => self.regs.csr.instreth = data,
                0x100 => self.regs.csr.sstatus = data,
                0x104 => self.regs.csr.sie = data,
                0x105 => self.regs.csr.stvec = data,
                //0x106 => self.regs.csr.scounteren = data,
                //0x10A => self.regs.csr.senvcfg = data,
                0x140 => self.regs.csr.sscratch = data,
                0x141 => self.regs.csr.sepc = data,
                0x142 => self.regs.csr.scause = data,
                0x143 => self.regs.csr.stval = data,
                0x144 => self.regs.csr.sip = data,
                0x180 => self.regs.csr.satp = data,
                0xF11 => self.regs.csr.mvendorid = data,
                0xF12 => self.regs.csr.marchid = data,
                0xF13 => self.regs.csr.mimpid = data,
                0xF14 => self.regs.csr.mhartid = data,
                0x300 => self.regs.csr.mstatus = data,
                0x301 => self.regs.csr.misa = data,
                0x302 => self.regs.csr.medeleg = data,
                0x303 => self.regs.csr.mideleg = data,
                0x304 => self.regs.csr.mie = data,
                0x305 => self.regs.csr.mtvec = data,
                0x306 => self.regs.csr.mcounteren = data,
                0x340 => self.regs.csr.sscratch = data,
                0x341 => self.regs.csr.mepc = data,
                0x342 => self.regs.csr.mcause = data,
                0x343 => self.regs.csr.mtval = data,
                0x344 => self.regs.csr.mip = data,
                _ => return Some(trap::Trap::IllegalInstruction),
            }
            return None;
        }
        return Some(trap::Trap::IllegalInstruction);
    }
    pub fn execute(&mut self) {
        while self.status {
            let instr: u32 = self.mem.read_word(self.regs.pc as usize);
            let decoded: RV32Instruction = decode::rv32_decode(instr);
            println!("[0x{:08X}]:<0x{:08X}> | got {:?}", self.regs.pc, instr, decoded);
            match decoded {
                RV32Instruction::Unknown => panic!("Invalid instruction"),
                RV32Instruction::Nop => {},
                RV32Instruction::RV32I(instr) => {
                    match instr {
                        RV32IInstruction::Lui(rd, imm) => {
                            self.regs.write(rd, imm as u32);
                        },
                        RV32IInstruction::Auipc(rd, imm) => {
                            self.regs.write(rd, self.regs.pc.wrapping_add_signed(imm));
                        },
                        RV32IInstruction::Jal(rd, imm) => {
                            self.regs.write(
                                if rd == 0 {
                                    1
                                } else {
                                    rd
                                },
                                self.regs.pc.wrapping_add(4)
                            );
                            self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                        },
                        RV32IInstruction::Jalr(rd, rs1, imm) => {
                            let t: u32 = self.regs.pc.wrapping_add(4);
                            self.regs.pc = self.regs.read(rs1).wrapping_add_signed(imm) & !1;
                            self.regs.write(
                                if rd == 0 {
                                    1
                                } else {
                                    rd
                                },
                                t
                            );
                        },
                        RV32IInstruction::Beq(rs1, rs2, imm) => {
                            if self.regs.read(rs1) == self.regs.read(rs2) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Bne(rs1, rs2, imm) => {
                            if self.regs.read(rs1) != self.regs.read(rs2) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Blt(rs1, rs2, imm) => {
                            if (self.regs.read(rs1) as i32) < (self.regs.read(rs2) as i32) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Bge(rs1, rs2, imm) => {
                            if (self.regs.read(rs1) as i32) >= (self.regs.read(rs2) as i32) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Bltu(rs1, rs2, imm) => {
                            if self.regs.read(rs1) < self.regs.read(rs2) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Bgeu(rs1, rs2, imm) => {
                            if self.regs.read(rs1) >= self.regs.read(rs2) {
                                self.regs.pc = self.regs.pc.wrapping_add_signed(imm);
                            }
                        },
                        RV32IInstruction::Lb(rd, rs1, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let ubyte: u8 = self.mem.read_byte(address as usize);
                            let idata: i32 = ((ubyte as i32) << 24) >> 24;
                            self.regs.write(rd, idata as u32);
                        },
                        RV32IInstruction::Lh(rd, rs1, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let uhalf: u16 = self.mem.read_half_word(address as usize);
                            let idata: i32 = ((uhalf as i32) << 16) >> 16;
                            self.regs.write(rd, idata as u32);
                        },
                        RV32IInstruction::Lw(rd, rs1, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let udata: u32 = self.mem.read_word(address as usize);
                            self.regs.write(rd, udata);
                        },
                        RV32IInstruction::Lbu(rd, rs1, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let ubyte: u8 = self.mem.read_byte(address as usize);
                            self.regs.write(rd, (ubyte as u32) & !0xFF);
                        },
                        RV32IInstruction::Lhu(rd, rs1, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let uhalf: u16 = self.mem.read_half_word(address as usize);
                            self.regs.write(rd, (uhalf as u32) & !0xFFFF);
                        },
                        RV32IInstruction::Sb(rs1, rs2, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let byte: u8 = (self.regs.read(rs2) & 0xFF) as u8;
                            self.mem.write_byte(address as usize, byte);
                        },
                        RV32IInstruction::Sh(rs1, rs2, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            let half: u16 = (self.regs.read(rs2) & 0xFFFF) as u16;
                            self.mem.write_half_word(address as usize, half);
                        },
                        RV32IInstruction::Sw(rs1, rs2, imm) => {
                            let address: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            self.mem.write_word(address as usize, self.regs.read(rs2));
                        },
                        RV32IInstruction::Addi(rd, rs1, imm) => {
                            let data: u32 = self.regs.read(rs1).wrapping_add_signed(imm);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Slti(rd, rs1, imm) => {
                            if (self.regs.read(rs1) as i32) < imm {
                                self.regs.write(rd, 1);
                            } else {
                                self.regs.write(rd, 0);
                            }
                        },
                        RV32IInstruction::Sltiu(rd, rs1, imm) => {
                            if self.regs.read(rs1) < imm as u32 {
                                self.regs.write(rd, 1);
                            } else {
                                self.regs.write(rd, 0);
                            }
                        },
                        RV32IInstruction::Xori(rd, rs1, imm) => {
                            let data: u32 = self.regs.read(rs1) ^ (imm as u32);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Ori(rd, rs1, imm) => {
                            let data: u32 = self.regs.read(rs1) | (imm as u32);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Andi(rd, rs1, imm) => {
                            let data: u32 = self.regs.read(rs1) & (imm as u32);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Slli(rd, rs1, shamt) => {
                            let data: u32 = self.regs.read(rs1) << shamt;
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Srli(rd, rs1, shamt) => {
                            let data: u32 = self.regs.read(rs1) >> shamt;
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Srai(/*rd, rs1, shamt*/_, _, _) => {
                            /*
                            let data: u32 = ((self.regs.read(rs1) as i32) >> shamt) as u32;
                            self.regs.write(rd, data);
                            */
                            panic!("Illegal instruction: SRAI");
                        },
                        RV32IInstruction::Add(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1).wrapping_add(self.regs.read(rs2));
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Sub(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1).wrapping_sub(self.regs.read(rs2));
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Sll(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1) << self.regs.read(rs2) & 0x1F;
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Slt(rd, rs1, rs2) => {
                            if (self.regs.read(rs1) as i32) < (self.regs.read(rs2) as i32) {
                                self.regs.write(rd, 1);
                            } else {
                                self.regs.write(rd, 0);
                            }
                        },
                        RV32IInstruction::Sltu(rd, rs1, rs2) => {
                            if self.regs.read(rs1) < self.regs.read(rs2) {
                                self.regs.write(rd, 1);
                            } else {
                                self.regs.write(rd, 0);
                            }
                        },
                        RV32IInstruction::Xor(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1) ^ self.regs.read(rs2);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Srl(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1) >> self.regs.read(rs2) & 0x1F;
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Sra(rd, rs1, rs2) => {
                            let data: u32 = ((self.regs.read(rs1) as i32) >> self.regs.read(rs2) & 0x1F) as u32;
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::Or(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1) | self.regs.read(rs2);
                            self.regs.write(rd, data);
                        },
                        RV32IInstruction::And(rd, rs1, rs2) => {
                            let data: u32 = self.regs.read(rs1) & self.regs.read(rs2);
                            self.regs.write(rd, data);
                        },
                        _ => panic!("Unimplmemented instruction"),
                    }
                },
                RV32Instruction::RV32M(instr) => match instr {
                    RV32MInstruction::Mul(rd, rs1, rs2) => {
                        let data: u32 = self.regs.read(rs1).wrapping_mul(self.regs.read(rs2));
                        self.regs.write(rd, data);
                    },
                    RV32MInstruction::Mulh(rd, rs1, rs2) => {
                        let data: i64 = (self.regs.read(rs1) as i32 * self.regs.read(rs2) as i32) as i64;
                        self.regs.write(rd, (data >> 32) as u32);
                    },
                    RV32MInstruction::Mulhsu(rd, rs1, rs2) => {
                        let data: i64 = self.regs.read(rs1) as i64 * self.regs.read(rs2) as u64 as i64;
                        self.regs.write(rd, (data >> 32) as u32);
                    },
                    RV32MInstruction::Mulhu(rd, rs1, rs2) => {
                        let data: u64 = self.regs.read(rs1) as u64 * self.regs.read(rs2) as u64;
                        self.regs.write(rd, (data >> 32) as u32);
                    },
                    RV32MInstruction::Div(rd, rs1, rs2) => {
                        if self.regs.read(rs2) == 0 {
                            panic!("Division by zero");
                        }
                        let data: i32 = self.regs.read(rs1) as i32 / self.regs.read(rs2) as i32;
                        self.regs.write(rd, data as u32);
                    },
                    RV32MInstruction::Divu(rd, rs1, rs2) => {
                        if self.regs.read(rs2) == 0 {
                            panic!("Division by zero");
                        }
                        let data: u32 = self.regs.read(rs1) / self.regs.read(rs2);
                        self.regs.write(rd, data);
                    },
                    RV32MInstruction::Rem(rd, rs1, rs2) => {
                        if self.regs.read(rs2) == 0 {
                            panic!("Division by zero");
                        }
                        let data: i32 = self.regs.read(rs1) as i32 % self.regs.read(rs2) as i32;
                        self.regs.write(rd, data as u32);
                    },
                    RV32MInstruction::Remu(rd, rs1, rs2) => {
                        if self.regs.read(rs2) == 0 {
                            panic!("Division by zero");
                        }
                        let data: u32 = self.regs.read(rs1) % self.regs.read(rs2);
                        self.regs.write(rd, data);
                    },
                },
                RV32Instruction::RV32A(instr) => match instr {
                    RV32AInstruction::ScW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        self.mem.write_word(address as usize, self.regs.read(rs2));
                        self.regs.write(rd, 0);
                    },
                    RV32AInstruction::AmoswapW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = self.regs.read(rs2);
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmoaddW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = self.regs.read(rs2) + t;
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmoxorW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = self.regs.read(rs2) ^ t;
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmoandW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = self.regs.read(rs2) & t;
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmoorW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = self.regs.read(rs2) | t;
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmominW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = if (self.regs.read(rs2) as i32) < (t as i32) {
                            self.regs.read(rs2)
                        } else {
                            t
                        };
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmomaxW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = if (self.regs.read(rs2) as i32) > (t as i32) {
                            self.regs.read(rs2)
                        } else {
                            t
                        };
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmominuW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = if self.regs.read(rs2) < t {
                            self.regs.read(rs2)
                        } else {
                            t
                        };
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                    RV32AInstruction::AmomaxuW(rd, rs1, rs2) => {
                        let address: u32 = self.regs.read(rs1);
                        let t: u32 = self.mem.read_word(address as usize);
                        let data: u32 = if self.regs.read(rs2) > t {
                            self.regs.read(rs2)
                        } else {
                            t
                        };
                        self.mem.write_word(address as usize, data);
                        self.regs.write(rd, t);
                    },
                },
                RV32Instruction::RV32Ziscr(instr) => match instr {
                    RV32ZicsrInstruction::Csrrw(rd, rs1, csr) => {
                        let t: u32 = self.read_csr(csr).unwrap(); // temporary error handling
                        let data: u32 = self.regs.read(rs1);
                        self.write_csr(csr, data).unwrap(); // temporary error handling
                        self.regs.write(rd, t);
                    },
                    RV32ZicsrInstruction::Csrrs(rd, rs1, csr) => {
                        let t: u32 = self.read_csr(csr).unwrap();
                        let data: u32 = t | self.regs.read(rs1);
                        self.write_csr(csr, data).unwrap();
                        self.regs.write(rd, t);
                    },
                    RV32ZicsrInstruction::Csrrc(rd, rs1, csr) => {
                        let t: u32 = self.read_csr(csr).unwrap();
                        let data: u32 = t & !self.regs.read(rs1);
                        self.write_csr(csr, data).unwrap();
                        self.regs.write(rd, t);
                    },
                    RV32ZicsrInstruction::Csrrwi(rd, zimm, csr) => {
                        self.regs.write(rd, self.read_csr(csr).unwrap());
                        self.write_csr(csr, (zimm & 0x1F) as u32);
                    },
                    RV32ZicsrInstruction::Csrrsi(rd, zimm, csr) => {
                        let t: u32 = self.read_csr(csr).unwrap();
                        let data: u32 = t | (zimm & 0x1F) as u32;
                        self.write_csr(csr, data);
                        self.regs.write(rd, t);
                    },
                    RV32ZicsrInstruction::Csrrci(rd, zimm, csr) => {
                        let t: u32 = self.read_csr(csr).unwrap();
                        let data: u32 = t & !(zimm & 0x1F) as u32;
                        self.write_csr(csr, data);
                        self.regs.write(rd, t);
                    },
                }, 
                RV32Instruction::TrapReturn(instr) => match instr {
                    TrapRetInstruction::Sret => {
                        self.regs.pc = self.read_csr(0x141).unwrap();
                        let mut sstatus: u32 = self.read_csr(0x100).unwrap();
                        let spp: u8 = ((sstatus >> 8) & 0x1) as u8; // get field SPP of sstatus
                        self.privilege = spp; // restore previous privilege from SPP
                        let spie: u8 = ((sstatus >> 5) & 0x1) as u8; // get field SPIE of sstatus
                        sstatus &= !2 & !(1 << 8); // clear SIE field and set SPP field to 0
                        sstatus |= ((spie << 1) | (1 << 5)) as u32; // restore field SIE of sstatus from SPIE and set SPIE to 1
                        self.write_csr(0x100, sstatus); // flush the updated sstatus back to the CSR
                    } ,
                    TrapRetInstruction::Mret => {
                        self.regs.pc = self.read_csr(0x341).unwrap();
                        let mut mstatus: u32 = self.read_csr(0x300).unwrap();
                        let mpp: u8 = ((mstatus >> 11) & 0x3) as u8;
                        self.privilege = mpp;
                        let mpie: u8 = ((mstatus >> 7) & 0x1) as u8;
                        mstatus &= (!8) & !(3 << 11);
                        mstatus |= ((mpie << 3) | (1 << 7)) as u32;
                        self.write_csr(0x300, mstatus);
                    },
                },
            }
            self.regs.pc = self.regs.pc.wrapping_add(4);
        }
    }
}
