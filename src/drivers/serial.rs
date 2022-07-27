// SPDX-License-Identifier: GPL-3.0-or-later

use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};

use crate::consts::serial::*;

pub struct Serial {
    pub uart: Uart<'static, usarts::UART>
}

impl Serial {
    
    pub fn new(
        tx: pins::TX,
        rx: pins::RX,
        uart: usarts::UART
    ) -> Self {
        let config = Config::default();
        let mut usart = Uart::new(uart, rx, tx, NoDma, NoDma, config);
        usart.blocking_write(b"Setting up uart\r\n").unwrap();
        info!("Setting up uart");

        Self { uart : usart }

        //let p = embassy_stm32::init(Default::default());
        //let mut usart = Uart::new(p.USART3, p.PD9, p.PD8, NoDma, NoDma, config);
    

    }
}
