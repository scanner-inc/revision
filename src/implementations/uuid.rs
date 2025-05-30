#![cfg(feature = "uuid")]

use super::super::Error;
use super::super::{DeserializeRevisioned, Revisioned, SerializeRevisioned};
use uuid::Uuid;

impl SerializeRevisioned for Uuid {
	#[inline]
	fn serialize_revisioned<W: std::io::Write>(&self, writer: &mut W) -> Result<(), Error> {
		writer.write_all(self.as_bytes()).map_err(Error::Io)
	}
}

impl DeserializeRevisioned for Uuid {
	#[inline]
	fn deserialize_revisioned<R: std::io::Read>(reader: &mut R) -> Result<Self, Error> {
		let mut v = [0u8; 16];
		reader.read_exact(&mut v).map_err(Error::Io)?;
		Uuid::from_slice(&v).map_err(|_| Error::Deserialize("invalid uuid".to_string()))
	}
}

impl Revisioned for Uuid {
	#[inline]
	fn revision() -> u16 {
		1
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_uuid() {
		#[rustfmt::skip]
        let val = Uuid::from_bytes([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        ]);
		let mut mem: Vec<u8> = vec![];
		val.serialize_revisioned(&mut mem).unwrap();
		assert_eq!(mem.len(), 16);
		let out =
			<Uuid as DeserializeRevisioned>::deserialize_revisioned(&mut mem.as_slice()).unwrap();
		assert_eq!(val, out);
	}
}
