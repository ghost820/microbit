#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    loop {}
}
