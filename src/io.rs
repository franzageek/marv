use std::io::Read;
use std::os::fd::AsRawFd;

pub struct KbdIn {
    stdin: std::io::Stdin,
    fd: std::os::fd::RawFd,
    termios: termios::Termios,
    termios_raw: termios::Termios,
    buf: [u8; 1],
}

impl KbdIn {
    pub fn new() -> KbdIn {
        let stdin: std::io::Stdin = std::io::stdin();
        let fd: std::os::fd::RawFd = stdin.as_raw_fd();
        let termios: termios::Termios = termios::Termios::from_fd(fd).unwrap();
        let mut termios_raw: termios::Termios = termios.clone();
        termios::cfmakeraw(&mut termios_raw);
        let flags: i32 = unsafe {
            libc::fcntl(fd, libc::F_GETFL)
        };
        unsafe {
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
        }
        return KbdIn {
            stdin: stdin,
            fd: fd,
            termios: termios,
            termios_raw: termios_raw,
            buf: [0u8; 1],
        };
    }
    pub fn try_read_byte(&mut self) -> Option<u8> {
        termios::tcsetattr(self.fd, termios::TCSANOW, &mut self.termios_raw).unwrap();
        let data: Option<u8>;
        match self.stdin.lock().read(&mut self.buf) {
            Ok(1) => data = Some(self.buf[0]),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => data = None, // no data available this cycle
            Err(e) => panic!("error reading stdin: {:?}", e),
            _ => data = None, // no data
        }
        termios::tcsetattr(self.fd, termios::TCSANOW, &mut self.termios).unwrap();
        return data;
    }
}

/*pub fn output_to_screen(cpu: &mut cpu::RiscV32) {
    while let Some(c) = cpu.uart.read(uart::UART_THR) {
        print!("{}", c as char);
    }
    std::io::stdout().flush().unwrap();
}*/
