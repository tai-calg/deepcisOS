use crate::{make_error, ErrorKind, Result};
use arrayvec::ArrayVec; //maxが決まっている固定長Vec
use bit_field::BitField;
use core::{fmt::{self}, ops::Range,};
use x86_64::instructions::port::{Port, self};

// ==== const or static ==== //

const INVALID_VENDER_ID :u16 = 0xffff;

static CONFIG : Config = Config(spin::Mutex::new
    (PortSet { addr: Port::new(0x0cf8), data: Port::new(0xcfc) }));


// ==== public ==== //

#[derive(Debug, Clone)]
pub(crate) struct Device {
    bus: u8,
    device: u8,
    function: u8,
    vender_id: u16,
    class_code: u32,
    header_type : u8,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}: vend {:04x}, class {:08x}, head {:02x}",
            self.bus, self.device, self.function, self.vender_id, self.class_code, self.header_type,
        )
    }
}

pub(crate) type Devices = ArrayVec<Device, 32>;

pub(crate) fn scan_all_bus()->Result<Devices> {
    let mut devices = Devices::new();

    let header_type = read_header_type(0,0,0);
    if is_single_function_device(header_type) {
        scan_bus(&mut devices , 0)?;
        return Ok(devices);
    }

    for bus in 1..8 {
        if read_vender_id(0, 0, bus) == INVALID_VENDER_ID { continue; }
        scan_bus(&mut devices, bus)?;
    }

    Ok(devices)
}

pub(crate) fn scan_bus(devices: &mut Devices, bus: u8) -> Result<()> {
    for device in 0..32 {
        if read_vender_id(bus, device, 0) == INVALID_VENDER_ID { continue; }
        scan_device(devices, bus, device)?;

    }
    Ok(())
}


// ==== private ==== //
fn scan_device(devices: &mut Devices,bus: u8, device: u8)-> Result<()> {
    scan_function(devices, bus, device, 0)?;
    if is_single_function_device(read_header_type(bus, device, 0))
    {
        return Ok(());
    }
    
    for function in 1..8 {
        if read_vender_id(bus, device, function) == INVALID_VENDER_ID { continue; }
        scan_function(devices, bus, device, function)?;
    }
    Ok(())
}
fn scan_function(devices: &mut Devices, bus: u8, device: u8, function: u8)-> Result<()> {
    let vender_id  = read_vender_id(bus, device, function);
    let class_code = read_class_code(bus, device, function);
    let header_type = read_header_type(bus, device, function);

    devices.try_push(Device{
        bus, device, function, vender_id, class_code, header_type,
    }).map_err(|_| make_error!(ErrorKind::Full))?;

    let base = ((class_code >> 24) & 0xff) as u8;
    let sub = ((class_code >> 16) & 0xff) as u8;

    if base == 0x06 && sub == 0x04 {
        // standart pci-pci bridge
        let bus_numbers = read_bus_number(bus, device, function);
        let secondary_bus = ((bus_numbers >> 8) & 0xff) as u8;
        scan_bus(devices, secondary_bus);
    }

    Ok(())
}

fn read_vender_id(bus: u8, device: u8, function: u8)-> u16 {
    let addr = ConfigAddr::new(bus,device ,function, 0x00);
    (CONFIG.read(addr) & 0xffff) as u16 
}
// fn read_device_id(bus: u8, device: u8, function: u8) -> u16 {
//     let addr = Addr::new(bus, device, function, 0x00);
//     (CONFIG.read(addr) >> 16) as u16
// }

fn read_header_type(bus: u8,device: u8, function: u8) -> u8 {
    let addr = ConfigAddr::new(bus, device, function, 0x0c);
    (CONFIG.read(addr) >> 16 & 0xff) as u8
} // & 0xffって意味あるの？
// ans : addr :u32 なのでまずそれを右16シフト。そして0xff = 0000000011111111なので末尾8bitだけ＆で抜き取ることができる。そこ以外はすべて０にされる
// なので マスクしてるときはマスク相手が何bitか気を付けて確認するといい。

fn read_class_code(bus: u8 , device: u8, function: u8) -> u32 {
    let addr = ConfigAddr::new(bus, device, function, 0x18);
    CONFIG.read(addr)
}

fn read_bus_number(bus: u8, device: u8, function: u8) -> u32 {
    let addr = ConfigAddr::new(bus, device, function, 0x18);
    CONFIG.read(addr)
}
fn is_single_function_device(header_type: u8) -> bool {
    (header_type & 0x80) == 0
}


// ==== struct or enum ==== //
struct ConfigAddr (u32);

impl ConfigAddr {
    const BITS_OFFSET : Range<usize> = 0..8;
    const BITS_FUNCTION : Range<usize> = 8..11;
    const BITS_DEVICE : Range<usize> = 11..16;
    const BITS_BUS : Range<usize> = 16..24;
    const BITS_RESERVED : Range<usize> = 24..31;
    const BITS_ENABLE : usize = 31;

    
    fn new(bus: u8, device : u8, function: u8, reg_addr:u8)-> Self {
        assert_eq!(reg_addr & 0x3, 0);
        let mut value = 0u32;
        value.set_bits(Self::BITS_OFFSET, u32::from(reg_addr)); 
        value.set_bits(Self::BITS_FUNCTION, u32::from(function));
        value.set_bits(Self::BITS_DEVICE, u32::from(device));
        value.set_bits(Self::BITS_BUS, u32::from(bus));
        value.set_bit(Self::BITS_ENABLE, true);
        Self(value)
    }
    // fn reg_addr(&self) -> u8 {
    //     self.0.get_bits(Self::BITS_ADDR) as u8
    // }
    // fn function(&self) -> u8 {
    //     self.0.get_bits(Self::BITS_FUNCTION) as u8
    // }
    // fn device(&self) -> u8 {
    //     self.0.get_bits(Self::BITS_DEVICE) as u8
    // }
    // fn bus(&self) -> u8 {
    //     self.0.get_bits(Self::BITS_BUS) as u8
    // }
    // fn enable(&self) -> bool {
    //     self.0.get_bit(Self::BIT_ENABLE)
    // }

}

#[derive(Debug)]
struct  PortSet {
    addr : Port<u32>,
    data : Port<u32>,
}

#[derive(Debug)]
struct Config(spin::Mutex<PortSet>);

impl Config {
    fn read(&self , addr: ConfigAddr)-> u32 {
        let mut ports = self.0.lock(); //self.0は一番目のフィールド
        unsafe {
            ports.addr.write(addr.0);
            ports.data.read()
        }
    }
    
    // fn write(&self, addr: Addr, data: u32) {
    //     let mut ports = self.0.lock();
    //     unsafe {
    //         ports.addr.write(addr.0);
    //         ports.data.write(data)
    //     }
    // }



}

