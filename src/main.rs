mod cpu;
mod instruction;
mod decode;
mod trap;

fn main() {
    println!("== MARV RISC-V RV32IMA EMULATOR v0.1 ==\n== written by <franzageek> ==");
    let mut marv: cpu::RiscV32 = cpu::RiscV32::new();
    marv.reset();
    marv.execute();
}
