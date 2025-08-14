use std::io::{Read, Write};

use colored::Colorize;

use crate::cpu;

pub fn rvll(cpu: &mut cpu::RiscV32, filename: &String) {
    let mut f = std::fs::File::open(&filename).expect("no devicetree blob found");
    let metadata = std::fs::metadata(&filename).expect("unable to read devicetree blob metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    let len: usize = cpu.mem.ram.len();
    let start_addr: usize = ((len as u64 - metadata.len()) - 0x1000) as usize;
    let end_addr: usize = (len - 0x1000) as usize;
    print!("{} -- RISC-V Linux Loader for MARV32IMA, v0.1 --", "[rvll]".purple());
    print!("{} loading devicetree blob at 0x{:08X}->0x{:08X}...", "[rvll]".purple(), start_addr, end_addr);
    std::io::stdout().flush().unwrap();
    unsafe {
        cpu.mem.ram[start_addr..end_addr].copy_from_slice(
            std::slice::from_raw_parts(
                buffer.as_ptr(), 
                metadata.len() as usize
            )
        );
    }
    println!("{}", "done".green());
    print!("{} loading hartid into x10 (a0)...", "[rvll]".purple());
    cpu.regs.x[10] = 0;
    println!("{}", "done".green());
    print!("{} loading devicetree blob address (0x{:08X}) into x11 (a1)...", "[rvll]".purple(), start_addr);
    cpu.regs.x[11] = start_addr as u32;
    println!("{}", "done".green());
}