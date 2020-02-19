//! Helpers for working with `~/.Xauthority`.

use std::io::Read;
use std::io::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AuthEntry {
    family: u16,
    address: Vec<u8>,
    number: Vec<u8>,
    name: Vec<u8>,
    data: Vec<u8>,
}

fn read_u16<R: Read>(read: &mut R) -> Result<u16, Error> {
    let mut buffer = [0; 2];
    read.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

fn read_string<R: Read>(read: &mut R) -> Result<Vec<u8>, Error> {
    let length = read_u16(read)?;
    let mut result = vec![0; length.into()];
    read.read_exact(&mut result[..])?;
    Ok(result)
}

pub(crate) fn read_entry<R: Read>(read: &mut R) -> Result<AuthEntry, Error> {
    let family = read_u16(read)?;
    let address = read_string(read)?;
    let number = read_string(read)?;
    let name = read_string(read)?;
    let data = read_string(read)?;
    Ok(AuthEntry { family, address, number, name, data })
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::{AuthEntry, read_entry};

    #[test]
    fn test_read() {
        // Data generated via xauth -f /tmp/file add :1 bar deadbeef
        let data = [
            0x01, 0x00, 0x00, 0x07, 0x5a, 0x77, 0x65, 0x69, 0x4c, 0x45,
            0x44, 0x00, 0x01, 0x31, 0x00, 0x03, 0x62, 0x61, 0x72, 0x00,
            0x04, 0xde, 0xad, 0xbe, 0xef,
        ];
        let mut cursor = Cursor::new(&data);
        let entry = read_entry(&mut cursor).unwrap();
        println!("{:?}", std::str::from_utf8(&entry.address));
        println!("{:?}", std::str::from_utf8(&entry.number));
        println!("{:?}", std::str::from_utf8(&entry.name));
        println!("{:?}", std::str::from_utf8(&entry.data));
        println!("{:x?}", entry.data);
        assert_eq!(entry, AuthEntry {
            family: 0x100,
            address: "ZweiLED".as_bytes().to_vec(),
            number: "1".as_bytes().to_vec(),
            name: "bar".as_bytes().to_vec(),
            data: u32::to_be_bytes(0xdeadbeef).to_vec(),
        });
    }
}
