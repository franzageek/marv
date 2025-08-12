pub mod rv32i;
pub mod rv32m;
pub mod rv32a;
pub mod rv32zicsr;

use crate::cpu;
use crate::trap;

pub trait Execute {
    fn execute(self, cpu: &mut cpu::RiscV32) -> Option<trap::Trap>;
}