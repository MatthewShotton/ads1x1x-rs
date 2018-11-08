use hal::i2c::{ Mock as I2cMock, Transaction as I2cTrans };
extern crate ads1x1x;
use self::ads1x1x::{ Ads1x1x, interface, ic, SlaveAddr, mode };

pub const DEVICE_ADDRESS : u8 = 0b100_1000;

pub struct Register;

impl Register {
    pub const CONVERSION : u8 = 0x00;
    pub const CONFIG     : u8 = 0x01;
    //const LOW_TH     : u8 = 0x02;
    //const HIGH_TH    : u8 = 0x03;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    //pub const OP_MODE      : u16 = 0b0000_0001_0000_0000;
    pub const OS           : u16 = 0b1000_0000_0000_0000;
    pub const DR2          : u16 = 0b0000_0000_1000_0000;
    pub const DR1          : u16 = 0b0000_0000_0100_0000;
    pub const DR0          : u16 = 0b0000_0000_0010_0000;
}

pub struct Config {
    pub bits: u16
}

impl Config {
    pub fn with_high(&self, mask: u16) -> Self {
        Config { bits: self.bits | mask }
    }
    pub fn with_low(&self, mask: u16) -> Self {
        Config { bits: self.bits & !mask }
    }

    pub fn msb(&self) -> u8 {
        (self.bits >> 8) as u8
    }

    pub fn lsb(&self) -> u8 {
        self.bits as u8
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0x8583 }
    }
}

macro_rules! impl_new_destroy {
    ($ic:ident, $create:ident, $destroy:ident, $trans:ty, $iface:ty) => {
        #[allow(unused)]
        pub fn $create(transactions: &[$trans]) -> Ads1x1x<$iface, ic::$ic, mode::OneShot> {
            Ads1x1x::$create(I2cMock::new(&transactions), SlaveAddr::default())
        }

        #[allow(unused)]
        pub fn $destroy<MODE>(dev: Ads1x1x<$iface, ic::$ic, MODE>) {
            dev.$destroy().done();
        }
    }
}

impl_new_destroy!(Ads1013, new_ads1013, destroy_ads1013, I2cTrans, interface::I2cInterface<I2cMock>);
impl_new_destroy!(Ads1113, new_ads1113, destroy_ads1113, I2cTrans, interface::I2cInterface<I2cMock>);

#[macro_export]
macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block.")
        }
    }
}

