use colored::Colorize;

pub struct RV32Memory {
    pub ram: Vec<u8>,
}

impl RV32Memory {
    pub fn new() -> RV32Memory {
        print!("initializing memory...");
        let ram: Vec<u8> = vec![0u8; (1u64 << 32) as usize];
        println!("{}, allocated {} bytes", "done".green(), ram.len().to_string().blue());
        return RV32Memory {
            ram: ram,
        };
    }

    pub fn read_byte(&mut self, address: usize) -> u8 {
        return self.ram[address];
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) { // maybe use the equivalent of memcpy for write ops
        self.ram[address] = byte;
        return;
    }

    pub fn read_half_word(&mut self, address: usize) -> u16 {
        return u16::from_le_bytes([self.ram[address], self.ram[address+1]]);
    }

    pub fn write_half_word(&mut self, address: usize, half: u16) {
        self.ram[address] = (half & 0xFF) as u8;
        self.ram[address+1] = ((half >> 8) & 0xFF) as u8;
        return;
    }

    pub fn read_word(&mut self, address: usize) -> u32 {
        return u32::from_le_bytes([self.ram[address], self.ram[address+1], self.ram[address+2], self.ram[address+3]]);
    }

    pub fn write_word(&mut self, address: usize, word: u32) {
        self.ram[address] = (word & 0xFF) as u8;
        self.ram[address+1] = ((word >> 8) & 0xFF) as u8;
        self.ram[address+2] = ((word >> 16) & 0xFF) as u8;
        self.ram[address+3] = ((word >> 24) & 0xFF) as u8;
        return;
    }

    pub fn read_double_word(&mut self, address: usize) -> u64 {
        return u64::from_le_bytes([self.ram[address], self.ram[address+1], self.ram[address+2], self.ram[address+3], self.ram[address+4], self.ram[address+5], self.ram[address+6], self.ram[address+7]]);
    }

    pub fn write_double_word(&mut self, address: usize, double: u64) {
        self.ram[address] = (double & 0xFF) as u8;
        self.ram[address+1] = ((double >> 8) & 0xFF) as u8;
        self.ram[address+2] = ((double >> 16) & 0xFF) as u8;
        self.ram[address+3] = ((double >> 24) & 0xFF) as u8;
        self.ram[address+4] = ((double >> 32) & 0xFF) as u8;
        self.ram[address+5] = ((double >> 40) & 0xFF) as u8;
        self.ram[address+6] = ((double >> 48) & 0xFF) as u8;
        self.ram[address+7] = ((double >> 56) & 0xFF) as u8;
        return;
    }
}
