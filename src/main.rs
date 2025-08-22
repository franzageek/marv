mod bootloader;
mod cpu;
mod decode;
mod extensions;
mod instruction;
mod interrupt;
mod io;
mod timer;
mod trap;
mod uart;
fn main() {
    println!("== MARV RISC-V RV32IMA EMULATOR v0.1 ==\n== written by <franzageek> ==");
    let mut marv: cpu::RiscV32 = cpu::RiscV32::new();
    marv.reset();
    bootloader::rvll(&mut marv, &String::from("src/devicetree/dtree.dtb"));
    marv.execute();
}
