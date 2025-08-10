mod cpu;
mod instruction;
mod decode;

fn main() {
    println!("== MARV RISC-V RV32I EMULATOR v0.1 ==\n== written by <franzageek> ==");
    let mut marv: cpu::RiscV32 = cpu::RiscV32::new();
    marv.reset();
    marv.execute();
}
