use std::io::Write;

use crate::cpu;
use crate::uart;

pub fn output_to_screen(cpu: &mut cpu::RiscV32) {
    print!("{}", cpu.uart.read(uart::UART_THR).unwrap() as char);
    std::io::stdout().flush().unwrap();
}


