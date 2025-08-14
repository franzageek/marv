use std::collections::VecDeque;

pub struct UART {
    pub rxtx: u8,
    pub lsr: u8,
    pub fifo: VecDeque<u8>,
}

const UART_BASE: usize = 0x1000_0000;
const UART_THR: usize = UART_BASE + 0x00;
const UART_RBR: usize = UART_BASE + 0x00;
const UART_LSR: usize = UART_BASE + 0x05;
const UART_END: usize = UART_BASE + 0x07;

pub fn match_addr(address: u32) -> bool {
   return address >= (UART_BASE as u32) && address <= (UART_END as u32);
}

impl UART {
    pub fn new() -> UART {
        return UART {
            rxtx: 0,
            lsr: 0,
            fifo: VecDeque::with_capacity(0),
        };
    }
    pub fn reset(&mut self) {
        self.rxtx = 0;
        self.lsr = 0x60; // transmitter holding register empty, data holding register empty
        self.fifo.reserve(16);
    }
    pub fn read(&mut self, address: u32) -> Option<u8> {
        match address as usize {
            UART_RBR => {
                if self.lsr & 0x1 != 0 {
                    let data: u8 = self.rxtx;
                    if self.fifo.len() > 0 {
                        self.rxtx = self.fifo.pop_front().unwrap();
                    } else {
                        self.lsr &= !0x1; // clear data ready bit
                        self.lsr |= 1 << 5; // set transmitter holding register empty
                    }
                    return Some(data);
                }
                return None; // no data ready
            },
            UART_LSR => return Some(self.lsr),
            _ => return None,
        }
    }
    pub fn write(&mut self, address: u32, data: u8) {
        match address as usize {
            UART_THR => {
                if self.lsr & (1 << 5) != 0  {
                    self.rxtx = data;
                    self.lsr &= !(1 << 5); // clear transmitter holding register empty bit
                } else {
                    self.fifo.push_back(data);
                }
                self.lsr |= 0x1; // set data ready bit
            },
            _ => {},
        }
    }
}