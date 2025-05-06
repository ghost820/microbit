#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod uart;

use core::fmt::Write as _;
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_io::{Read, Write};
use heapless::String;
use microbit::board::Board;
use microbit::hal::uarte::{Baudrate, Parity, Uarte};
use panic_halt as _;

use uart::UartePort;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let tx_buf = cortex_m::singleton!(TX_BUF: [u8; 1] = [0u8; 1]).unwrap();
    let rx_buf = cortex_m::singleton!(RX_BUF: [u8; 1] = [0u8; 1]).unwrap();
    let (tx, rx) = serial.split(tx_buf, rx_buf).unwrap();
    let mut uarte_port = UartePort { tx, rx };

    loop {
        if let Ok(res) = uarte_port.read_until::<128>(b'\n') {
            let mut iter = res.split_ascii_whitespace();
            match iter.next().unwrap_or_default() {
                "/test" => {
                    let _ = writeln!(uarte_port, "OK");
                }
                _ => {
                    let _ = writeln!(uarte_port, "Unknown command");
                }
            }
        }
    }
}
