use core::fmt;
use embedded_io::Read;
use heapless::String;
use microbit::hal::uarte::{Instance, UarteRx, UarteTx};

pub struct UartePort<T: Instance> {
    pub tx: UarteTx<T>,
    pub rx: UarteRx<T>,
}

pub enum UartError {
    ReceiveError,
}

impl<T: Instance> UartePort<T> {
    pub fn read_until<const U: usize>(&mut self, delim: u8) -> Result<String<U>, UartError> {
        let mut result = String::<U>::new();

        let mut buf = [0u8; 1];
        loop {
            let Ok(1) = self.rx.read(&mut buf) else {
                return Err(UartError::ReceiveError);
            };
            let _ = result.push(buf[0] as char);
            if buf[0] == delim {
                break;
            }
        }

        Ok(result)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}
