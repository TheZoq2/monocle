use core::cmp::min;
use usb_device::class_prelude::*;
use usb_device::utils::AtomicMutex;
use usb_device::Result;

pub const USB_CLASS_CDC: u8 = 0x02;
const USB_CLASS_DATA: u8 = 0x0a;
const CDC_SUBCLASS_ACM: u8 = 0x02;
const CDC_PROTOCOL_AT: u8 = 0x01;

const CS_INTERFACE: u8 = 0x24;
const CDC_TYPE_HEADER: u8 = 0x00;
const CDC_TYPE_CALL_MANAGEMENT: u8 = 0x01;
const CDC_TYPE_ACM: u8 = 0x02;
const CDC_TYPE_UNION: u8 = 0x06;

const REQ_SET_LINE_CODING: u8 = 0x20;
const REQ_SET_CONTROL_LINE_STATE: u8 = 0x22;

struct Buf {
    buf: [u8; 64],
    len: usize,
}

pub struct SerialPort<'a, B: 'a + UsbBus + Sync> {
    comm_if: InterfaceNumber,
    comm_ep: EndpointIn<'a, B>,
    data_if: InterfaceNumber,
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,

    read_buf: AtomicMutex<Buf>,
}

impl<'a, B: UsbBus + Sync> SerialPort<'a, B> {
    pub fn new(bus: &'a UsbBusWrapper<B>) -> SerialPort<'a, B> {
        SerialPort {
            comm_if: bus.interface(),
            comm_ep: bus.interrupt(8, 255),
            data_if: bus.interface(),
            read_ep: bus.bulk(64),
            write_ep: bus.bulk(64),
            read_buf: AtomicMutex::new(Buf {
                buf: [0; 64],
                len: 0,
            }),
        }
    }

    pub fn write(&self, data: &[u8]) -> Result<usize> {
        match self.write_ep.write(data) {
            Ok(count) => Ok(count),
            Err(UsbError::Busy) => Ok(0),
            e => e,
        }
    }

    pub fn read(&self, data: &mut [u8]) -> Result<usize> {
        let mut guard = self.read_buf.try_lock();

        let buf = match guard {
            Some(ref mut buf) => buf,
            None => { return Ok(0) },
        };

        // Terrible buffering implementation for brevity's sake

        if buf.len == 0 {
            buf.len = match self.read_ep.read(&mut buf.buf) {
                Ok(count) => count,
                Err(UsbError::NoData) => return Ok(0),
                e => return e,
            };
        }

        if buf.len == 0 {
            return Ok(0);
        }

        let count = min(data.len(), buf.len);

        &data[..count].copy_from_slice(&buf.buf[0..count]);

        buf.buf.rotate_left(count);
        buf.len -= count;

        Ok(count)
    }
}

impl<'a, B: UsbBus + Sync> UsbClass for SerialPort<'a, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        // TODO: make a better DescriptorWriter to make it harder to make invalid descriptors
        writer.interface(
            self.comm_if,
            1,
            USB_CLASS_CDC,
            CDC_SUBCLASS_ACM,
            CDC_PROTOCOL_AT)?;

        writer.write(
            CS_INTERFACE,
            &[CDC_TYPE_HEADER, 0x10, 0x01])?;

        writer.write(
            CS_INTERFACE,
            &[CDC_TYPE_CALL_MANAGEMENT, 0x00, self.data_if.into()])?;

        writer.write(
            CS_INTERFACE,
            &[CDC_TYPE_ACM, 0x00])?;

        writer.write(
            CS_INTERFACE,
            &[CDC_TYPE_UNION, self.comm_if.into(), self.data_if.into()])?;

        writer.endpoint(&self.comm_ep)?;

        writer.interface(
            self.data_if,
            2,
            USB_CLASS_DATA,
            0x00,
            0x00)?;

        writer.endpoint(&self.write_ep)?;
        writer.endpoint(&self.read_ep)?;

        Ok(())
    }

    fn control_out(&self, req: &control::Request, buf: &[u8]) -> ControlOutResult {
        let _ = buf;

        if req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface
        {
            return match req.request {
                REQ_SET_LINE_CODING => ControlOutResult::Ok,
                REQ_SET_CONTROL_LINE_STATE => ControlOutResult::Ok,
                _ => ControlOutResult::Ignore,
            };
        }

        ControlOutResult::Ignore
    }
}
