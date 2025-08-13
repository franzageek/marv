use crate::cpu;

pub const CLINT_BASE: usize = 0x0200_0000;
pub const CLINT_MTIME: usize = CLINT_BASE + 0xBFF8;
pub const CLINT_MTIMECMP: usize = CLINT_BASE + 0x4000;

fn check_cmp(time: u64, cpu: &mut cpu::RiscV32) {
    if time >= cpu.mem.read_double_word(CLINT_MTIMECMP) {
        cpu.regs.csr.mip |= 1 << 7;
    } else {
        cpu.regs.csr.mip &= !(1 << 7);
    }
    return;
}

pub fn update(cpu: &mut cpu::RiscV32) {
    let time: u64 = cpu.mem.read_double_word(CLINT_MTIME).wrapping_add(1);
    cpu.mem.write_double_word(CLINT_MTIME, time);
    check_cmp(time, cpu);
    return;
}