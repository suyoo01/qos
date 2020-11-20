use volatile_register::{RO, RW};
use crate::{println, print};

/// Initialize uart
/// Reference: Zynq-7000 SOC TRM
pub unsafe fn uart_init() {
    let uart = &mut *UART_BASE;
    if uart.cr.read() & (1<<4) != 0 { return }
    uart.cr.write(1 << 5); // Set no parity
    uart.cr.write(1 << 3 | 1 << 5); // Disable rx and tx

    //TODO: Baudrate configuration
    uart.cr.write(1 << 1 | 1); // Soft reset rx and tx data path
    uart.cr.write(1 << 2 | 1 << 4); // Enable rx and tx
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        let uart = &mut *UART_BASE;
        uart.write_fmt(args).unwrap();
    }
}


/// Wrapper for UartRegs
pub struct Uart;
impl Uart {
    fn get(&self) -> &mut UartRegs {
        unsafe {
            &mut *(UART_BASE as *mut UartRegs)
        }
    }
}

pub fn read() -> u8 {
    unsafe {
        let uart = &mut *UART_BASE;
        uart.read()
    }
}




#[repr(C)]
pub struct UartRegs {
    pub cr: RW<u32>,          // Control Register
    pub mr: RW<u32>,          // Mode Register
    pub ier: RW<u32>,         // Interrupt Enable
    pub idr: RW<u32>,         // Interrupt Disable
    pub imr: RO<u32>,         // Interrupt Mask
    pub isr: RW<u32>,         // Channel Interrupt Status
    pub baudgen: RW<u32>,     // Baud Rate
    pub rx_tout: RW<u32>,     // Receiver Timeout
    pub rxwm: RW<u32>,        // Receiver FIFO Trigger level
    pub modem_cr: RW<u32>,    // Modem Control
    pub modem_st: RW<u32>,    // Modem Status
    pub sr: RO<u32>,          // Channel status
    pub fifo: RW<u32>,        // Transmit and recieve
    pub baudgen_div: RW<u32>, // Baud Rate Divder
    pub flow_delay: RW<u32>,  // Flow Control Delay
    pub tx_trigger: RW<u32>,  // Transmitter FIFO Trigger level
}

impl UartRegs {
    pub fn write(&mut self, c: u8) {
        while self.is_tx_full() {} // Polling
        unsafe {
            self.fifo.write(c as u32);
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.bytes() {
            self.write(c);
        }    
    }

    pub fn read(&mut self) -> u8 {
        while self.is_rx_empty() {}
        self.fifo.read() as u8
    }

    pub fn is_tx_full(&self) -> bool {
        (self.sr.read() & (1<<4)) != 0
    }

    pub fn is_tx_empty(&self) -> bool {
        (self.sr.read() & (1<<3)) != 0
    }

    pub fn is_rx_full(&self) -> bool {
        (self.sr.read() & (1<<2)) != 0
    }

    pub fn is_rx_empty(&self) -> bool {
        (self.sr.read() & (1<<1)) != 0
    }
}

impl core::fmt::Write for UartRegs {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_str(s);
        Ok(())
    }
}


const _UART_PHYS: usize = 0xe0001000;
const UART_BASE: *mut UartRegs = 0xfff00000 as *mut UartRegs;


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\r\n"));
    ($($arg:tt)*) => (print!("{}\r\n", format_args!($($arg)*)));
}
