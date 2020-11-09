use volatile_register::{RW, RO};

#[repr(C)]
pub struct UartRegs {
    pub cr: RW<u32>, // Control Register
    pub mr: RW<u32>, // Mode Register
    pub ier: RW<u32>, // Interrupt Enable
    pub idr: RW<u32>, // Interrupt Disable
    pub imr: RO<u32>, // Interrupt Mask
    pub isr: RW<u32>, // Channel Interrupt Status
    pub baudgen: RW<u32>, // Baud Rate
    pub rx_tout: RW<u32>, // Receiver Timeout
    pub rxwm: RW<u32>, // Receiver FIFO Trigger level
    pub modem_cr: RW<u32>, // Modem Control
    pub modem_st: RW<u32>, // Modem Status
    pub sr: RW<u32>, // Channel status
    pub fifo: RW<u32>, // Transmit and recieve
    pub baudgen_div: RW<u32>, // Baud Rate Divder
    pub flow_delay: RW<u32>, // Flow Control Delay
    pub tx_trigger: RW<u32> // Transmitter FIFO Trigger level
}

static _UART_PHYS: usize = 0xe0001000;
static UART_BASE: usize = 0xfff00000 as usize;

/// Initialize uart
/// Reference: Zynq-7000 SOC TRM
pub unsafe fn uart_init() {
    let uart = &mut *(UART_BASE as *mut usize as *mut UartRegs);
    uart.cr.write(1<<5); // Set no parity
    uart.cr.write(1<<3 | 1<<5); // Disable rx and tx

//TODO: Baudrate configuration

    uart.cr.write(1<<1 | 1); // Soft reset rx and tx data path
    uart.cr.write(1<<2 | 1<<4); // Enable rx and tx
}

pub struct Uart;

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        write_str(s);
        Ok(())
    }
}

pub fn write(c: u32) {
    unsafe {
        let uart = &mut *(UART_BASE as *mut UartRegs);
        uart.fifo.write(c);
    }
}

pub fn write_str(s: &str) {
    for c in s.bytes() {
        write(c as u32);
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut uart = Uart{};
    uart.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::uart::_print(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

