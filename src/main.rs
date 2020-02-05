#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use cortex_m::peripheral::syst::SystClkSource;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use ssd1306::prelude::DisplaySize::Display128x32 as DisplaySize;
use core::fmt::Write;

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c
};

#[entry]
fn main() -> ! {
    
    if let (Some(mut p), Some(_cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
              
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

        // Set up the display
        let mut disp: TerminalMode<_> = SSD1306Builder::new().with_size(DisplaySize).connect_i2c(i2c).into();
        
        disp.init().unwrap();
        disp.flush().unwrap();
        
        for c in 97..123 {
            disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) }).unwrap(); 
        }
               
    
        loop {
                for c in 97..123 {
                disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) }).unwrap(); 
            }
        }
        
    });
    
}

    loop {continue;}
    
}
