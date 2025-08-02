mod cpu;
mod instruction;

fn main() {
    println!("== MARV RISC-V RV32IM EMULATOR v0.1 ==\n== written by <franzageek> ==");
    let mut marv: cpu::RiscV = cpu::RiscV::new();
    marv.regs.pc = 0;
}
