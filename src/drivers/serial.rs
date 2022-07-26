// SPDX-License-Identifier: GPL-3.0-or-later

use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};

use crate::consts::serial::*;

pub struct Serial {
    pub tx: pins::TX,
    pub rx: pins::RX,
    pub uart: usarts::UART
}

impl Serial {
    
    pub fn new(
        tx: pins::TX,
        rx: pins::RX,
        uart: usarts::UART
    ) -> Self {
        // let config = Config::default();
        // let mut usart = Uart::new(uart, tx, rx, NoDma, NoDma, config);
        // unwrap!(usart.blocking_write(b"Setting up uart\r\n"));
        // info!("wrote Setting up uart");

        Self { tx, rx, uart}

        //let p = embassy_stm32::init(Default::default());
        //let mut usart = Uart::new(p.USART3, p.PD9, p.PD8, NoDma, NoDma, config);
    

    }
}
