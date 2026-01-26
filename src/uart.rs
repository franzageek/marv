use std::io::Write;

//use std::collections::VecDeque;
use crate::io;

pub struct UART {
    //pub rxtx: u8,
    //pub lsr: u8,
    pub kbd: io::KbdIn,
    //pub fifo: VecDeque<u8>, // disable FIFO
}

pub const UART_BASE: u32 = 0x1000_0000;
pub const UART_THR: u32 = UART_BASE + 0x00;
pub const UART_RBR: u32 = UART_BASE + 0x00;
pub const UART_LSR: u32 = UART_BASE + 0x05;
pub const UART_END: u32 = UART_BASE + 0x07;

const THRE_TEMT: u8 = (1 << 6) | (1 << 5);
const DR: u8 = 1 << 0;

pub fn match_addr(address: u32) -> bool {
   return address >= (UART_BASE as u32) && address <= (UART_END as u32);
}

impl UART {
    pub fn new() -> UART {
        return UART {
        /*rxtx: 0,
            lsr: 0,
            fifo: VecDeque::with_capacity(0),*/ // disable FIFO
            kbd: io::KbdIn::new()
        };
    }
    pub fn reset(&mut self) {
        /*self.rxtx = 0;
        self.lsr = THRE_TEMT; // transmitter holding register empty, data holding register empty
        self.fifo.reserve(16); // disable FIFO*/
    }
    pub fn read(&mut self, address: u32) -> Option<u8> {
        let data: Option<u8> = self.kbd.try_read_byte();
        match address {
            UART_RBR => {
                return data;
                /*if self.lsr & DR != 0 {
                    let data: u8 = self.rxtx;
                    if self.fifo.len() > 0 {
                        self.rxtx = self.fifo.pop_front().unwrap();
                    } else {
                        self.lsr &= !DR; // clear data ready bit
                        self.lsr |= THRE_TEMT; // set transmitter holding register empty
                    }
                    //self.lsr &= !DR; // clear data ready bit
                    //self.lsr |= THRE_TEMT; // set transmitter holding register empty
                    return Some(data);
                }

                return None; // no data ready*/


            },
            UART_LSR => {
                return Some(THRE_TEMT | if let Some(_) = data {
                    DR
                } else {
                    0
                });
            },
            _ => return None,
        }
    }
    pub fn write(&mut self, address: u32, data: u8) {
        match address {
            UART_THR => {
                /*if self.lsr & THRE_TEMT != 0  {
                    self.rxtx = data;
                    self.lsr &= !THRE_TEMT; // clear transmitter holding register empty bit
                } else {
                    self.fifo.push_back(data);
                } // disable FIFO
                self.lsr |= DR; // set data ready bit*/
                print!("{}", data as char);
                std::io::stdout().flush().unwrap();
            },
            _ => {},
        }
    }
}
