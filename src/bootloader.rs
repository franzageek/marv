use std::io::{Read, Write};

use colored::Colorize;

use crate::cpu;
use crate::timer;

pub struct BootloaderInfo {
    pub dtb: String,
    pub kernelimg: String,
}

impl BootloaderInfo {
    pub fn from(dtb: String, kernelimg: String) -> BootloaderInfo {
        BootloaderInfo {
            dtb,
            kernelimg,
        }
    }
}

fn read_into_buffer(filename: &String) -> Vec<u8> {
    let mut f = std::fs::File::open(filename).expect("file not found");
    let metadata: std::fs::Metadata = std::fs::metadata(filename).expect("unable to read file metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    return buffer;
}

fn write_to_ram(ram: &mut Vec<u8>, buffer: &Vec<u8>, start_addr: usize) {
    unsafe {
        ram[start_addr..(start_addr + buffer.len())].copy_from_slice(
            std::slice::from_raw_parts(
                buffer.as_ptr(),
                buffer.len() as usize
            )
        );
    }
}

pub fn rvll(cpu: &mut cpu::RiscV32, blinfo: BootloaderInfo) {
    println!("{} -- RISC-V Linux Loader for MARV32IMA, v0.1 --", "[rvll]".purple());
    let len: usize = cpu.mem.ram.len();
    let mut start_addr: usize;
    let mut end_addr: usize;
    {
        let buffer = read_into_buffer(&blinfo.dtb);
        start_addr = (len - buffer.len()) - 0x1000;
        end_addr = len - 0x1000;
        print!("{} loading devicetree blob at 0x{:08X}->0x{:08X}...", "[rvll]".purple(), start_addr, end_addr);
        std::io::stdout().flush().unwrap();
        write_to_ram(&mut cpu.mem.ram, &buffer, start_addr);
    }
    println!("{}", "done".green());
    print!("{} loading hartid into x10 (a0)...", "[rvll]".purple());
    cpu.regs.x[10] = 0;
    println!("{}", "done".green());
    print!("{} loading devicetree blob address (0x{:08X}) into x11 (a1)...", "[rvll]".purple(), start_addr);
    cpu.regs.x[11] = start_addr as u32;
    println!("{}", "done".green());
    print!("{} resetting timer...", "[rvll]".purple());
    cpu.mem.write_double_word(timer::CLINT_MTIMECMP, 0xFFFFFFFF_FFFFFFFF);
    let data: u64 = cpu.mem.read_double_word(timer::CLINT_MTIMECMP) as u64;
    assert_eq!(data, 0xFFFFFFFF_FFFFFFFF);
    println!("{}", "done".green());
    {
        let buffer: Vec<u8> = read_into_buffer(&blinfo.kernelimg);
        start_addr = 0x80000000;
        end_addr = start_addr + buffer.len();
        print!("{} loading kernel image at 0x{:08X}->0x{:08X}...", "[rvll]".purple(), start_addr, end_addr);
        std::io::stdout().flush().unwrap();
        write_to_ram(&mut cpu.mem.ram, &buffer, start_addr);
    }
    println!("{}", "done".green());
    print!("{} setting PC to 0x{:08X}...", "[rvll]".purple(), start_addr);
    cpu.regs.pc = start_addr as u32;
    println!("{}", "done".green());
    println!("{} starting execution of kernel image...", "[rvll]".purple());
    return;
}
