use crate::bits::{BitReader, BitWriter};
use crate::codebook::build_codebook;
use crate::frequency::count_frequencies;
use crate::tree::{build_tree, HuffmanNode};

// Magic number that identifies a valid .huff file
const MAGIC: &[u8; 4] = b"HUFF";

// Compress

pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
	if data.is_empty() {
		return Err("Cannot compress empty input".into());
	}

	// 1. Count frequencies and build the tree + codebook
	let freq     = count_frequencies(data);
	let tree     = build_tree(&freq).ok_or("Failed to build Huffman tree")?;
	let codebook = build_codebook(&tree);

	// 2. Encode the payload into bits
	let mut writer = BitWriter::new();
	for &byte in data {
		let code = &codebook[byte as usize];
		if code.is_empty() {
			return Err(format!("No code found for byte {}", byte));
		}
		writer.write_bits(code);
	}
	let payload = writer.finish();

	// 3. Assemble the .huff binary format
	// Total bits in the payload (needed to strip padding on decode)
	let total_bits: u64 = data.iter()
		.map(|&b| codebook[b as usize].len() as u64)
		.sum();

	let mut out = Vec::new();

	// Header: MAGIC (4 bytes)
	out.extend_from_slice(MAGIC);

	// Header: original file size (8 bytes, little-endian u64)
	out.extend_from_slice(&(data.len() as u64).to_le_bytes());

	// Header: total encoded bit count (8 bytes, little-endian u64)
	out.extend_from_slice(&total_bits.to_le_bytes());

	// Header: frequency table (256 × 8 bytes = 2048 bytes)
	for &f in freq.iter() {
		out.extend_from_slice(&f.to_le_bytes());
	}

	// Payload: the compressed bitstream
	out.extend_from_slice(&payload);

	Ok(out)
}

// Decompress

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
	// Minimum valid size: 4 (magic) + 8 (orig_size) + 8 (bits) + 2048 (freq)
	const HEADER_SIZE: usize = 4 + 8 + 8 + 2048;

	if data.len() < HEADER_SIZE {
		return Err("Input too small to be a valid .huff file".into());
	}

	// 1. Validate magic number
	if &data[0..4] != MAGIC {
		return Err("Invalid magic number — not a .huff file".into());
	}

	// 2. Read original size
	let orig_size = u64::from_le_bytes(data[4..12].try_into().unwrap()) as usize;

	// 3. Read total encoded bit count
	let total_bits = u64::from_le_bytes(data[12..20].try_into().unwrap());

	// 4. Read frequency table (256 × u64)
	let mut freq = [0u64; 256];
	for i in 0..256 {
		let offset = 20 + i * 8;
		freq[i] = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
	}

	// 5. Rebuild the Huffman tree from the frequency table
	let tree = build_tree(&freq).ok_or("Failed to rebuild Huffman tree")?;

	// 6. Decode the bitstream
	let payload = &data[HEADER_SIZE..];
	let mut reader = BitReader::new(payload, total_bits);
	let mut output = Vec::with_capacity(orig_size);

	while output.len() < orig_size {
		let byte = decode_one(&tree, &mut reader)
			.ok_or("Unexpected end of bitstream during decoding")?;
		output.push(byte);
	}

	Ok(output)
}

/// Walk the Huffman tree one bit at a time until we reach a leaf.
fn decode_one(root: &HuffmanNode, reader: &mut BitReader) -> Option<u8> {
	let mut node = root;
	loop {
		match node {
			HuffmanNode::Leaf { byte, .. } => return Some(*byte),
			HuffmanNode::Internal { left, right, .. } => {
				let bit = reader.read_bit()?;
				node = if bit { right } else { left };
			}
		}
	}
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compress_decompress_text() {
		let original = b"hello world, this is a huffman test!";
		let compressed   = compress(original).unwrap();
		let decompressed = decompress(&compressed).unwrap();
		assert_eq!(original.to_vec(), decompressed);
	}

	#[test]
	fn test_compressed_is_smaller_for_repetitive_data() {
		// Header is 2068 bytes fixed cost, so input must be large enough
		// that the 1-bit payload (single unique byte) beats the overhead.
		// 40,000 'a's , payload is ~40,000 bits = 5,000 bytes
		// Total compressed ≈ 2068 + 5000 = 7068 bytes << 40,000 bytes original
		let original: Vec<u8> = vec![b'a'; 40_000];
		let compressed = compress(&original).unwrap();
		assert!(
			compressed.len() < original.len(),
			"Compressed ({} bytes) should be smaller than original ({} bytes)",
			compressed.len(), original.len()
		);
	}

	#[test]
	fn test_small_file_has_header_overhead() {
		// For tiny inputs the fixed 2068-byte header dominates —
		// this is a known trade-off of storing the full frequency table.
		// Document it explicitly rather than pretending it doesn't exist.
		let original = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 40 bytes
		let compressed = compress(original).unwrap();
		assert!(
			compressed.len() > original.len(),
			"Small files are expected to grow due to the fixed header overhead"
		);
	}

	#[test]
	fn test_invalid_magic_rejected() {
		let bad = b"NOPE\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
		assert!(decompress(bad).is_err());
	}

	#[test]
	fn test_binary_data_roundtrip() {
		let original: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
		let compressed   = compress(&original).unwrap();
		let decompressed = decompress(&compressed).unwrap();
		assert_eq!(original, decompressed);
	}

	#[test]
	fn test_empty_input_rejected() {
		assert!(compress(&[]).is_err());
	}
}