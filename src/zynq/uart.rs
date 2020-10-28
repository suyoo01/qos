use volatile_register::{RW, RO};

#[repr(C)]
pub struct uart_regs {
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