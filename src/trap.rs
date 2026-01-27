use crate::{cpu, extensions::Execute};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Trap {
    MisalignedInstructionAddress = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    MisalignedLoadAddr = 4,
    LoadAccessFault = 5,
    MisalignedStoreAddr = 6 ,
    StoreAccessFault = 7,
    UModeEnvCall = 8,
    SModeEnvCall = 9,
    Res0 = 10,
    MModeEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    Res1 = 14,
    StorePageFault = 15,
    DoubleTrap = 16,
    Res2 = 17,
    SoftwareCheck = 18,
    HardwareError = 19,
}

impl Trap {
    fn handle_smode(cpu: &mut cpu::RiscV32, cause: u32, val: u32) {
        cpu.regs.csr.sepc = cpu.regs.pc;
        cpu.regs.csr.scause = cause;
        cpu.regs.csr.stval = val;
        let sie: u8 = ((cpu.regs.csr.sstatus >> 1) & 0x1) as u8; // get field SIE of sstatus
        cpu.regs.csr.sstatus &= !((1 << 8) | (1 << 5) | (1 << 1)); // clear SPP, SIE and SPIE
        cpu.regs.csr.sstatus |= (sie << 5) as u32; // set SPIE to previous value of SIE
        cpu.regs.csr.sstatus |= ((cpu.privilege & 0x1) as u32) << 8; // set SPP to current privilege
        cpu.privilege = 1; // set privilege to S-mode
        cpu.regs.pc = cpu.regs.csr.stvec & !0x3; // set PC to stvec, aligned to 4 bytes
    }

    fn handle_mmode(cpu: &mut cpu::RiscV32, cause: u32, val: u32) {
        cpu.regs.csr.mepc = cpu.regs.pc;
        cpu.regs.csr.mcause = cause;
        cpu.regs.csr.mtval = val;
        let sie: u8 = ((cpu.regs.csr.mstatus >> 1) & 0x1) as u8;
        cpu.regs.csr.mstatus &= !((3 << 8) | (1 << 5) | (1 << 1));
        cpu.regs.csr.mstatus |= (sie << 5) as u32;
        cpu.regs.csr.mstatus |= ((cpu.privilege & 0x3) as u32) << 8;
        cpu.privilege = 3;
        cpu.regs.pc = cpu.regs.csr.mtvec & !0x3;
    }

    pub fn take(code: Trap, cpu: &mut cpu::RiscV32, val: u32) -> Trap {
        if cpu.regs.csr.medeleg & (1 << code as u32) > 0 {
            Trap::handle_smode(cpu, code as u32, val);
        } else {
            Trap::handle_mmode(cpu, code as u32, val);
        }
        return code;
    }

    pub fn display(self, cpu: &cpu::RiscV32) {
        match self { // [ ] find a way to show additional information (like mtval and privilege), maybe using enum for name and struct for data
            Trap::MisalignedInstructionAddress => eprintln!("[EXCEPTION] Misaligned instruction address"),
            Trap::InstructionAccessFault => eprintln!("[EXCEPTION] Instruction access fault"),
            Trap::IllegalInstruction => eprintln!("[EXCEPTION] Illegal instruction at PC: [0x{:08X}]\n{}", if cpu.privilege == 3 { cpu.regs.csr.mepc } else { cpu.regs.csr.sepc }, cpu),
            Trap::Breakpoint => eprintln!("[EXCEPTION] Breakpoint"),
            Trap::MisalignedLoadAddr => eprintln!("[EXCEPTION] Misaligned load address"),
            Trap::LoadAccessFault => eprintln!("[EXCEPTION] Load access fault"),
            Trap::MisalignedStoreAddr => eprintln!("[EXCEPTION] Misaligned store address"),
            Trap::StoreAccessFault => eprintln!("[EXCEPTION] Store access fault"),
            Trap::UModeEnvCall => eprintln!("[EXCEPTION] User mode environment call"),
            Trap::SModeEnvCall => eprintln!("[EXCEPTION] Supervisor mode environment call"),
            Trap::Res0 => eprintln!("[EXCEPTION] Reserved"),
            Trap::MModeEnvCall => eprintln!("[EXCEPTION] Machine mode environment call"),
            Trap::InstructionPageFault => eprintln!("[EXCEPTION] Instruction page fault"),
            Trap::LoadPageFault => eprintln!("[EXCEPTION] Load page fault"),
            Trap::Res1 => eprintln!("[EXCEPTION] Reserved"),
            Trap::StorePageFault => eprintln!("[EXCEPTION] Store page fault"),
            Trap::DoubleTrap => eprintln!("[EXCEPTION] Double trap"),
            Trap::Res2 => eprintln!("[EXCEPTION] Reserved"),
            Trap::SoftwareCheck => eprintln!("[EXCEPTION] Software check"),
            Trap::HardwareError => eprintln!("[EXCEPTION] Hardware error"),
        }
    }
}

#[derive(Debug)]
pub enum TrapRetInstruction {
    Sret,
    Mret,
}

impl Execute for TrapRetInstruction {
    fn execute(self, cpu: &mut crate::cpu::RiscV32) -> Option<self::Trap> {
        match self {
            TrapRetInstruction::Sret => {
                cpu.regs.pc = cpu.read_csr(0x141).unwrap();
                let mut sstatus: u32 = cpu.read_csr(0x100).unwrap();
                let spp: u8 = ((sstatus >> 8) & 0x1) as u8; // get field SPP of sstatus
                cpu.privilege = spp; // restore previous privilege from SPP
                let spie: u8 = ((sstatus >> 5) & 0x1) as u8; // get field SPIE of sstatus
                sstatus &= !(2 | (1 << 8)); // clear SIE field and set SPP field to 0
                sstatus |= ((spie << 1) | (1 << 5)) as u32; // restore field SIE of sstatus from SPIE and set SPIE to 1
                cpu.write_csr(0x100, sstatus)?; // flush the updated sstatus back to the CSR
                return None;
            },
            TrapRetInstruction::Mret => {
                cpu.regs.pc = cpu.read_csr(0x341).unwrap();
                let mut mstatus: u32 = cpu.read_csr(0x300).unwrap();
                let mpp: u8 = ((mstatus >> 11) & 0x3) as u8;
                cpu.privilege = mpp;
                let mpie: u8 = ((mstatus >> 7) & 0x1) as u8;
                mstatus &= !(8 | (3 << 11));
                mstatus |= ((mpie << 3) | (1 << 7)) as u32;
                cpu.write_csr(0x300, mstatus)?;
                return None;
            },
        }
    }
}
