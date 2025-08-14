use crate::cpu;

pub fn check(cpu: &mut cpu::RiscV32) {
    let pending_m: u32 = cpu.regs.csr.mie & cpu.regs.csr.mip & !cpu.regs.csr.mideleg;
    let pending_s: u32 = cpu.regs.csr.sie & cpu.regs.csr.sip & cpu.regs.csr.mideleg;
    let mut i: i8 = 11;
    while i > 0 {
        if (pending_m & (1 << i) != 0) && (cpu.regs.csr.mstatus & (1 << 3) > 0) {
            cpu.regs.csr.mepc = cpu.regs.pc; // set MEPC to current PC
            cpu.regs.csr.mcause = (1 << 31) | (1 << i); // set cause and interrupt bits (1 << 31) and (1 << i) of MCAUSE
            cpu.regs.csr.mtval = 0; // set MTVAL to 0
            cpu.regs.csr.mstatus &= !((3 << 11) | (1 << 7) | (1 << 3)); // clear MPP and MPIE, then set MIE to 0
            cpu.regs.csr.mstatus |= (((cpu.privilege & 0x3) as u32) << 11) as u32; // set MPP to current privilege level
            let mie: u8 = ((cpu.regs.csr.mstatus >> 3) & 0x1) as u8; // get field MIE of mstatus
            cpu.regs.csr.mstatus |= (mie << 7) as u32; // set MPIE to previous value of MIE
            cpu.privilege = 3; // set privilege to M-mode
            cpu.regs.pc = cpu.regs.csr.mtvec /* & !0x3  // for alignment */; // set PC to interrupt handler installed by guest OS
        }
        if (pending_s & (1 << i) != 0) && (cpu.regs.csr.sstatus & (1 << 1) > 0) {
            cpu.regs.csr.sepc = cpu.regs.pc;
            cpu.regs.csr.scause = (1 << 31) | (1 << i);
            cpu.regs.csr.stval = 0;
            cpu.regs.csr.sstatus &= !((1 << 8) | (1 << 5) | (1 << 1));
            cpu.regs.csr.sstatus |= (((cpu.privilege & 0x1) as u32) << 8) as u32;
            let sie: u8 = ((cpu.regs.csr.sstatus >> 1) & 0x1) as u8;
            cpu.regs.csr.sstatus |= (sie << 5) as u32;
            cpu.privilege = 1;
            cpu.regs.pc = cpu.regs.csr.stvec /* & !0x3  // for alignment */;
        }
        i -= 2;
    }
}