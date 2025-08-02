pub struct Regs {
    pub x: [u32; 32],
    pub pc: u32,
}

pub struct Memory {
    pub ram: Vec<u8>,
}

pub struct RiscV {
    pub regs: Regs,
    pub mem: Memory,
}

impl Regs {
    pub fn new() -> Regs {
        return Regs {
            x: [0u32; 32],
            pc: 0,
        };
    }
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            ram: vec![0u8; 2 ^ 32],
        };
    }
}

impl RiscV {
    pub fn new() -> RiscV {
        return RiscV {
            regs: Regs::new(),
            mem: Memory::new(),
        };
    }
}
