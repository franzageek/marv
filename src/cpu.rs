use crate::decode;
use crate::extensions::Execute;
use crate::interrupt;
use crate::memory::RV32Memory;
use crate::timer;
use crate::trap;
use crate::uart::UART;
use crate::instruction::*;
use std::fmt;
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

pub struct RiscV32 {
    pub regs: RV32Regs,
    pub mem: RV32Memory,
    pub uart: UART,
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
impl std::fmt::Display for RV32Regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "\npc:[0x{:08X}]\n(x1|ra):[0x{:08X}] (x2|sp):[0x{:08X}] (x3|gp):[0x{:08X}] (x4|tp):[0x{:08X}]\n(x5|t0):[0x{:08X}] (x6|t1):[0x{:08X}] (x7|t2):[0x{:08X}]\n(x8|s0):[0x{:08X}] (x9|s1):[0x{:08X}]\n(x10|a0):[0x{:08X}] (x11|a1):[0x{:08X}] (x12|a2):[0x{:08X}] (x13|a3):[0x{:08X}]\n(x14|a4):[0x{:08X}] (x15|a5):[0x{:08X}] (x16|a6):[0x{:08X}] (x17|a7):[0x{:08X}]\n(x18|s2):[0x{:08X}] (x19|s3):[0x{:08X}] (x20|s4):[0x{:08X}] (x21|s5):[0x{:08X}] (x22|s6):[0x{:08X}]\n(x23|s7):[0x{:08X}] (x24|s8):[0x{:08X}] (x25|s9):[0x{:08X}] (x26|s10):[0x{:08X}] (x27|s11):[0x{:08X}]\n(x28|t3):[0x{:08X}] (x29|t4):[0x{:08X}] (x30|t5):[0x{:08X}] (x31|t6):[0x{:08X}]",
            self.pc, self.x[1], self.x[2], self.x[3], self.x[4], self.x[5], self.x[6], self.x[7], self.x[8], self.x[9], self.x[10], self.x[11], self.x[12], self.x[13], self.x[14], self.x[15], self.x[16], self.x[17], self.x[18], self.x[19], self.x[20], self.x[21], self.x[22], self.x[23], self.x[24], self.x[25], self.x[26], self.x[27], self.x[28], self.x[29], self.x[30], self.x[31]
        )
    }
}

#[allow(dead_code)]
impl RiscV32 {
    pub fn new() -> RiscV32 {
        return RiscV32 {
            regs: RV32Regs::new(),
            mem: RV32Memory::new(),
            uart: UART::new(),
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
        print!("setting hardware thread ID...");
        self.regs.csr.mhartid = 0;
        println!("{}", "done".green());
        print!("resetting UART...");
        self.uart.reset();
        println!("{}, TX line is empty, transmitter is empty", "done".green());
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
    pub fn read_csr(&mut self, csr: u16) -> Result<u32, trap::Trap> {
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
                0x3A0..=0x3EF => return Ok(0), // implementation specific CSRs
                _ => return Err(trap::Trap::take(trap::Trap::IllegalInstruction, self, self.regs.pc)),
            }
        }
        return Err(trap::Trap::take(trap::Trap::IllegalInstruction, self, self.regs.pc));
    }
    pub fn write_csr(&mut self, csr: u16, data: u32) -> Option<trap::Trap> {
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
                0x3A0..=0x3EF => {}, // implementation specific CSRs
                _ => return Some(trap::Trap::take(trap::Trap::IllegalInstruction, self, self.regs.pc)),
            }
            return None;
        }
        return Some(trap::Trap::take(trap::Trap::IllegalInstruction, self, self.regs.pc));
    }

    pub fn execute(&mut self) {
        let mut instr: u32;
        let mut decoded: RV32Instruction;
        while self.status {
            /*if let Some(c) = kbd.try_read_byte() {
                self.uart.write(uart::UART_THR, c);
            }*/
            instr = self.mem.read_word(self.regs.pc as usize);
            decoded = decode::rv32_decode(instr);
            //self.uart.kbd.try_read_byte();
            eprintln!("[0x{:08X}]:<0x{:08X}> | got {:?}", self.regs.pc, instr, decoded);
            match decoded.execute(self) {
                None => {},
                Some(trap) => {
                    trap.display(self);
                    panic!("Emulation halted"); // [ ] display some data like regs, memory
                },
            }
            self.regs.pc = self.regs.pc.wrapping_add(4);
            //io::output_to_screen(self);
            timer::update(self);
            interrupt::check(self);
        }
    }
}

impl std::fmt::Display for RiscV32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "privilege level: {}\nregs: {{ {}\n}}", self.privilege, self.regs)
    }
}
