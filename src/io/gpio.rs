use volatile_register::{RO, RW};


#[repr(C)]
pub struct GPIO{
    pub msk0_lsw: RW<u32>,          // Control Register
    pub msk0_msw: RW<u32>,          // Mode Register
    pub msk1_lsw: RW<u32>,         // Interrupt Enable
    pub msk1_msw: RW<u32>,         // Interrupt Disable
    pub msk2_lsw: RW<u32>,         // Interrupt Mask
    pub msk2_msw: RW<u32>,         // Channel Interrupt Status
    pub msk3_lsw: RW<u32>,     // Baud Rate
    pub msk3_msw: RW<u32>,     // Receiver Timeout
    pub dum1: [RO<u32>;8],//20~3c
    pub data: [RW<u32>;4], //40~4c    
    pub dum2: [RO<u32>;4],//50~5c
    pub data_ro: [RO<u32>;4], //60~6c
    pub dum3: [RO<u32>;101],//70~200
    pub dirm0: RW<u32>,//204
    pub outen0: RW<u32>,//208
    pub dum4: [RO<u32>;14],//20c~240
    pub dirm1: RW<u32>,//244
    pub outen1: RW<u32>,//248
    pub dum5: [RO<u32>;14],
    pub dirm2: RW<u32>,//284
    pub outen2: RW<u32>,//288
    pub dum6: [RO<u32>;14],
    pub dirm3: RW<u32>,
    pub outen3: RW<u32>
}

#[repr(C)]
pub struct GPIOmode{
    pub dir_mod: RW<u32>,
    pub outen: RW<u32>
}

static GPIO_BASE: usize = 0xfff01000 as usize;

/// Initialize MIO
/// Reference: Zynq-7000 SOC TRM
pub unsafe fn gpio_init() {
    let hello = "finish gpio init";
    let gpio = &mut *(GPIO_BASE as *mut GPIO);

    /*MIO configuraiton*/
    gpio.dirm0.write(1<<7);
    gpio.outen0.write(1<<7);
    gpio.dirm2.write(0xF0);
    gpio.outen2.write(0xF0);
    /*test for mio*/
    (*gpio).data[0].write(1<<7);
    io::uart::write_str(hello);
    /*init EMIO*/
    (*gpio).data[2].write(0);

}

/*push button[i] -> led[i] on*/

pub unsafe fn toggle(){
    let gpio = &mut *(GPIO_BASE as *mut GPIO);
    let push ="push";

    if (*gpio).data_ro[2].read()==0x1 {
        (*gpio).data[2].write((*gpio).data_ro[2].read()|0x10);
        io::uart::write_str(push);
    }
    else if (*gpio).data_ro[2].read()==0x2 {
        (*gpio).data[2].write((*gpio).data_ro[2].read()|0x20);
        io::uart::write_str(push);
    }
    else{
        (*gpio).data[2].write((*gpio).data_ro[2].read()&0xF);

    }
}
pub unsafe fn led_on(channel:u32){
    let gpio = &mut *(GPIO_BASE as *mut GPIO);
    let led_ch = (1<<4)<<(channel);
    (*gpio).data[2].write((*gpio).data_ro[2].read()|led_ch);
}

pub unsafe fn led_off(channel:u32){
    let gpio = &mut *(GPIO_BASE as *mut GPIO);
    let led_off = !((1<<4)<<(channel));
}
pub unsafe fn mled_on(){
    let gpio = &mut *(GPIO_BASE as *mut GPIO);
    let led_ch = 1<<7;
    (*gpio).data[0].write((*gpio).data_ro[0].read()|led_ch);
}
pub unsafe fn mled_off(){
    let gpio = &mut *(GPIO_BASE as *mut GPIO);
    let led_ch = !(1<<7);
    (*gpio).data[0].write((*gpio).data_ro[0].read()&led_ch);
}
//#![feature(asm)]
pub unsafe fn delay() {
    //for _ in 1..1000{
    asm!(
          "
          mov r3, #0
          .test3:
          mov r2, #0
          .test2:
          mov r1, #0
          .test:
          cmp r1, #1000
          bge .endfor
          nop
          add r1, r1, #1
          b .test
          .endfor:
          cmp r2, #1000
          bge .endfor2
          nop
          add r2, r2, #1
          b .test2
          .endfor2: 
          cmp r3, #100
          bge .endfor3
          nop
          add r3, r3, #1
          b .test3
          .endfor3:
          "
          );
}
      
