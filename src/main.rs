#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod uart;

use core::fmt::Write as _;
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_io::ReadReady;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::hal::uarte::{Baudrate, Instance, Parity, Uarte};
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

    let mut matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    display.set_delay_ms(1);

    loop {
        display.show(&mut timer, matrix, 5);

        if uarte_port.rx.read_ready().unwrap_or(false) {
            if let Ok(res) = uarte_port.read_until::<128>(b'\n') {
                let mut iter = res.split_ascii_whitespace();
                match iter.next().unwrap_or_default() {
                    "/matrix" => handle_matrix(&mut uarte_port, &mut iter, &mut matrix),
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
}

fn handle_matrix<T: Instance>(
    uarte_port: &mut UartePort<T>,
    iter: &mut core::str::SplitAsciiWhitespace,
    matrix: &mut [[u8; 5]; 5],
) {
    const ERR_MSG: &str = "Usage: /matrix <v1> <v2> ... <v25> where v = [0,1]";

    let mut i = 0;
    let mut j = 0;
    for e in iter {
        let Ok(val) = e.parse::<u8>() else {
            let _ = writeln!(uarte_port, "{}", ERR_MSG);
            return;
        };
        if val > 1 {
            let _ = writeln!(uarte_port, "{}", ERR_MSG);
            return;
        }
        matrix[i][j] = val;
        j += 1;
        if j == 5 {
            j = 0;
            i += 1;
        }
        if i == 5 {
            break;
        }
    }
    let _ = writeln!(uarte_port, "OK");
}
