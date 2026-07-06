// BitWriter

/// Packs individual bits into a Vec<u8>, MSB first.
/// Call `finish()` to flush the final partial byte (zero-padded).
pub struct BitWriter {
	bytes: Vec<u8>,
	current_byte: u8,   // bits accumulate here
	bit_count: u8,   // how many bits are filled in current_byte (0–7)
}

impl BitWriter {
	pub fn new() -> Self {
		BitWriter {
			bytes: Vec::new(),
			current_byte: 0,
			bit_count: 0,
		}
	}

	pub fn write_bit(&mut self, bit: bool) {
		// Shift current byte left and OR in the new bit
		self.current_byte = (self.current_byte << 1) | (bit as u8);
		self.bit_count += 1;

		if self.bit_count == 8 {
			self.bytes.push(self.current_byte);
			self.current_byte = 0;
			self.bit_count = 0;
		}
	}

	pub fn write_bits(&mut self, bits: &[bool]) {
		for &bit in bits {
			self.write_bit(bit);
		}
	}

	/// Flush any remaining bits zero-padded to a full byte.
	pub fn finish(mut self) -> Vec<u8> {
		if self.bit_count > 0 {
			// Shift remaining bits to the MSB side
			self.current_byte <<= 8 - self.bit_count;
			self.bytes.push(self.current_byte);
		}
		self.bytes
	}
}

// BitReader

/// Reads individual bits from a byte slice, MSB first.
/// Stops at `total_bits` to ignore zero-padding at the end.
pub struct BitReader<'a> {
	bytes: &'a [u8],
	byte_index: usize,  // which byte we're currently reading
	bit_index: u8,     // which bit within that byte (0 = MSB, 7 = LSB)
	bits_read: u64,    // total bits consumed so far
	total_bits: u64,    // stop reading after this many bits
}

impl<'a> BitReader<'a> {
	pub fn new(bytes: &'a [u8], total_bits: u64) -> Self {
		BitReader {
			bytes,
			byte_index: 0,
			bit_index: 0,
			bits_read: 0,
			total_bits,
		}
	}

	/// Returns the next bit, or None if we've reached total_bits or end of data.
	pub fn read_bit(&mut self) -> Option<bool> {
		if self.bits_read >= self.total_bits {
			return None;
		}
		if self.byte_index >= self.bytes.len() {
			return None;
		}

		// Extract bit from MSB side: shift right by (7 - bit_index)
		let bit = (self.bytes[self.byte_index] >> (7 - self.bit_index)) & 1 == 1;

		self.bit_index += 1;
		self.bits_read += 1;

		if self.bit_index == 8 {
			self.bit_index = 0;
			self.byte_index += 1;
		}

		Some(bit)
	}
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_write_read_roundtrip() {
		let bits = vec![true, false, true, true, false, false, true, false];
		let mut writer = BitWriter::new();
		writer.write_bits(&bits);
		let bytes = writer.finish();

		let mut reader = BitReader::new(&bytes, bits.len() as u64);
		let recovered: Vec<bool> = std::iter::from_fn(|| reader.read_bit()).collect();
		assert_eq!(bits, recovered);
	}

	#[test]
	fn test_partial_byte_padded_correctly() {
		// Write only 3 bits: 1, 0, 1 should become 10100000 = 0xA0
		let mut writer = BitWriter::new();
		writer.write_bit(true);
		writer.write_bit(false);
		writer.write_bit(true);
		let bytes = writer.finish();
		assert_eq!(bytes.len(), 1);
		assert_eq!(bytes[0], 0b10100000);
	}

	#[test]
	fn test_total_bits_stops_reading_padding() {
		let mut writer = BitWriter::new();
		writer.write_bits(&[true, false, true]); // 3 real bits
		let bytes = writer.finish();             // padded to 1 byte

		// Reader must stop at 3 bits, not read the 5 padding zeros
		let mut reader = BitReader::new(&bytes, 3);
		assert_eq!(reader.read_bit(), Some(true));
		assert_eq!(reader.read_bit(), Some(false));
		assert_eq!(reader.read_bit(), Some(true));
		assert_eq!(reader.read_bit(), None); // padding is ignored
	}
}