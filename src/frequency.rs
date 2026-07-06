/// Counts occurrences of every byte value (0–255) in `data`.
/// Returns a fixed-size array of 256 u64 counts.
/// Index = byte value, value = how many times it appeared.
pub fn count_frequencies(data: &[u8]) -> [u64; 256] {
	let mut freq = [0u64; 256];
	for &byte in data {
		freq[byte as usize] += 1;
	}
	freq
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_empty_input() {
		let freq = count_frequencies(&[]);
		assert!(freq.iter().all(|&f| f == 0));
	}

	#[test]
	fn test_single_byte() {
		let freq = count_frequencies(&[42]);
		assert_eq!(freq[42], 1);
		assert_eq!(freq[0], 0);
	}

	#[test]
	fn test_repeated_bytes() {
		let data = vec![0u8, 0, 1, 2, 2, 2];
		let freq = count_frequencies(&data);
		assert_eq!(freq[0], 2);
		assert_eq!(freq[1], 1);
		assert_eq!(freq[2], 3);
	}

	#[test]
	fn test_all_bytes() {
		// Every byte 0-255 appears exactly once
		let data: Vec<u8> = (0..=255).collect();
		let freq = count_frequencies(&data);
		assert!(freq.iter().all(|&f| f == 1));
	}
}