use std::io::{Write, Error as IOError};
use std::convert::TryInto;
use std::error::Error;
use byteorder::{NativeEndian, WriteBytesExt};

pub trait ToWireFormat<W: Write> {
    fn to_wire(&self, writer: &mut W) -> Result<(), Box<dyn Error>>;
}

pub struct SetupRequest {
    byte_order: u8,
    pad0: u8,
    protocol_major_version: u16,
    protocol_minor_version: u16,
    pad1: [u8; 2],
    authorization_protocol_name: [u8; 0],
    authorization_protocol_data: [u8; 0],
}

trait WritePaddingExt: Write {
    fn write_padding(&mut self, length: usize) -> Result<(), IOError> {
        let padding = 4 - length % 4;
        let padding = if padding == 4 { 0 } else { padding };
        match padding {
            3 => self.write(&[0, 0, 0])?,
            2 => self.write(&[0, 0])?,
            1 => self.write(&[0])?,
            _ => 0,
        };
        Ok(())
    }
}

/// All types that implement `Write` get methods defined in `WritePaddingExt` for free.
impl<W: Write + ?Sized> WritePaddingExt for W {}

impl<W: Write> ToWireFormat<W> for SetupRequest {
    fn to_wire(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write(&[self.byte_order, self.pad0])?;
        writer.write_u16::<NativeEndian>(self.protocol_major_version)?;
        writer.write_u16::<NativeEndian>(self.protocol_minor_version)?;
        writer.write_u16::<NativeEndian>(self.authorization_protocol_name.len().try_into()?)?;
        writer.write_u16::<NativeEndian>(self.authorization_protocol_data.len().try_into()?)?;
        writer.write(&self.pad1)?;
        writer.write(&self.authorization_protocol_name);
        writer.write_padding(self.authorization_protocol_name.len())?;
        writer.write(&self.authorization_protocol_data);
        writer.write_padding(self.authorization_protocol_data.len())?;
        Ok(())
    }
}
