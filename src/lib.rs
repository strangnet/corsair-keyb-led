extern crate cuesdk_sys as sys;

#[derive(Debug)]
pub struct Driver {}

#[derive(Debug)]
pub enum Error {
    Handshake,
}

impl Driver {
    pub fn init(debug: bool) -> Result<Driver, Error> {
        unsafe {
            let hs = sys::CorsairPerformProtocolHandshake();
            println!("{:?}", hs)
        }

        if debug {
            return Err(Error::Handshake);
        }

        Ok(Driver {})
    }
}
