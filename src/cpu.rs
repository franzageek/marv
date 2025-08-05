use crate::instruction;
use crate::instruction::RV32IInstruction;
use colored::Colorize;

pub struct RV32Regs {
    pub x: [u32; 32],
    pub pc: u32,
}

pub struct RV32Memory {
    pub ram: Vec<u8>,
}

pub struct RiscV32 {
    pub regs: RV32Regs,
    pub mem: RV32Memory,
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

impl RiscV32 {
    pub fn new() -> RiscV32 {
        return RiscV32 {
            regs: RV32Regs::new(),
            mem: RV32Memory::new(),
            status: false
        };
    }
    pub fn reset(&mut self) {
        print!("setting processor state...");
        self.status = true;
        println!("{}", "done".green());
        print!("resetting program counter...");
        self.regs.pc = 0;
        println!("{}, execution starts at <0x{:08X}>", "done".green(), self.regs.pc);
        print!("clearing X registers...");
        self.regs.x.fill(0);
        println!("{}, all X registers have been set to 0", "done".green());
        print!("clearing RAM memory...");
        self.mem.ram.fill(0);
        println!("{}", "done".green());
        println!("{}", "successful RV32 processor reset".on_truecolor(0, 100, 0));
    }
    pub fn execute(&mut self) {
        while self.status {
            let instr: u32 = self.mem.read_word(self.regs.pc as usize);
            let decoded: RV32IInstruction = instruction::rv32_decode(instr);
            println!("[PC: <0x{:08X}>] got {:?}", self.regs.pc, decoded);
            match decoded {
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
                RV32IInstruction::Unknown => {
                    panic!("Invalid instruction");
                }
                _ => {
                    panic!("Unimplmemented instruction");
                }
            }
            self.regs.pc = self.regs.pc.wrapping_add(4);
        }
    }
}
