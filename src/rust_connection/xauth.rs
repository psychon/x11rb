//! Helpers for working with `~/.Xauthority`.

use std::io::{Read, Error, ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Family {
    Local,
    // Wild, <- this seems to be encoded as 0x10000, which does not fit a u16
    Netname,
    Krb5Principal,
    LocalHost,
    Unknown(u16),
}

impl From<u16> for Family {
    fn from(value: u16) -> Self {
        match value {
            256 => Family::Local,
            254 => Family::Netname,
            253 => Family::Krb5Principal,
            252 => Family::LocalHost,
            value => Family::Unknown(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AuthEntry {
    family: Family,
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

pub(crate) fn read_entry<R: Read>(read: &mut R) -> Result<Option<AuthEntry>, Error> {
    let family = match read_u16(read) {
        Ok(family) => family,
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }.into();
    let address = read_string(read)?;
    let number = read_string(read)?;
    let name = read_string(read)?;
    let data = read_string(read)?;
    Ok(Some(AuthEntry { family, address, number, name, data }))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::{AuthEntry, read_entry, Family};

    #[test]
    fn test_read() {
        // Data generated via xauth -f /tmp/file add :1 bar deadbeef
        let data = [
            0x01, 0x00, 0x00, 0x07, 0x5a, 0x77, 0x65, 0x69, 0x4c, 0x45,
            0x44, 0x00, 0x01, 0x31, 0x00, 0x03, 0x62, 0x61, 0x72, 0x00,
            0x04, 0xde, 0xad, 0xbe, 0xef,
        ];
        let mut cursor = Cursor::new(&data[..]);
        let entry = read_entry(&mut cursor).unwrap();
        assert_eq!(entry, Some(AuthEntry {
            family: Family::Local,
            address: "ZweiLED".as_bytes().to_vec(),
            number: "1".as_bytes().to_vec(),
            name: "bar".as_bytes().to_vec(),
            data: u32::to_be_bytes(0xdeadbeef).to_vec(),
        }));
    }

    #[test]
    fn test_read_iterate() {
        // Data generated via:
        //   xauth -f /tmp/file add :1 bar deadbeef
        //   xauth -f /tmp/file add 1.2.3.4:2 baz aabbccdd
        let data = [
            0x01, 0x00, 0x00, 0x07, 0x5a, 0x77, 0x65, 0x69, 0x4c, 0x45,
            0x44, 0x00, 0x01, 0x31, 0x00, 0x03, 0x62, 0x61, 0x72, 0x00,
            0x04, 0xde, 0xad, 0xbe, 0xef, 0x00, 0x00, 0x00, 0x04, 0x01,
            0x02, 0x03, 0x04, 0x00, 0x01, 0x32, 0x00, 0x03, 0x62, 0x61,
            0x7a, 0x00, 0x04, 0xaa, 0xbb, 0xcc, 0xdd
        ];
        let mut cursor = Cursor::new(&data[..]);
        for expected in &[
           AuthEntry {
               family: Family::Local,
               address: "ZweiLED".as_bytes().to_vec(),
               number: "1".as_bytes().to_vec(),
               name: "bar".as_bytes().to_vec(),
               data: u32::to_be_bytes(0xdeadbeef).to_vec(),
           },
           AuthEntry {
               family: Family::Unknown(0), // No idea why this is Unknown
               address: vec![1, 2, 3, 4],
               number: "2".as_bytes().to_vec(),
               name: "baz".as_bytes().to_vec(),
               data: u32::to_be_bytes(0xaabbccdd).to_vec(),
           },
        ] {
            let entry = read_entry(&mut cursor).unwrap();
            assert_eq!(entry.as_ref(), Some(expected));
        }
        let entry = read_entry(&mut cursor).unwrap();
        assert_eq!(entry, None);
    }
}
