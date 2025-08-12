use crate::extensions::Execute;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Trap {
    MisalignedInstructionAddress,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    MisalignedLoadAddr,
    LoadAccessFault,
    MisalignedStoreAddr,
    StoreAccessFault,
    UModeEnvCall,
    HSModeEnvCall,
    VSModeEnvCall,
    MModeEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    InstructionGuestPageFault,
    LoadGuestPageFault,
    VirtualInstruction,
    StoreGuestPageFault,
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
                sstatus &= !2 & !(1 << 8); // clear SIE field and set SPP field to 0
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
                mstatus &= (!8) & !(3 << 11);
                mstatus |= ((mpie << 3) | (1 << 7)) as u32;
                cpu.write_csr(0x300, mstatus)?;
                return None;
            },
        }
    }
}