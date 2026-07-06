use crate::tree::HuffmanNode;

/// Maps each byte value (0–255) to its Huffman bit sequence.
/// Bytes that never appear in the input have an empty Vec.
pub type Codebook = [Vec<bool>; 256];

/// Walks the Huffman tree recursively, building the codebook.
pub fn build_codebook(root: &HuffmanNode) -> Codebook {
	// Rust requires this dance to initialise a [Vec<bool>; 256]
	// because Vec<bool> doesn't implement Copy
	let mut codebook: Codebook = std::array::from_fn(|_| Vec::new());
	let mut path = Vec::new();
	traverse(root, &mut path, &mut codebook);
	codebook
}

/// Recursive DFS traversal of the Huffman tree.
fn traverse(node: &HuffmanNode, path: &mut Vec<bool>, codebook: &mut Codebook) {
	match node {
		HuffmanNode::Leaf { byte, .. } => {
			// Assign current path as this byte's code
			codebook[*byte as usize] = path.clone();
		}
		HuffmanNode::Internal { left, right, .. } => {
			// Go left append 0
			path.push(false);
			traverse(left, path, codebook);
			path.pop();

			// Go right append 1
			path.push(true);
			traverse(right, path, codebook);
			path.pop();
		}
	}
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::frequency::count_frequencies;
	use crate::tree::build_tree;

	#[test]
	fn test_all_used_bytes_have_codes() {
		let data = b"abracadabra";
		let freq  = count_frequencies(data);
		let tree  = build_tree(&freq).unwrap();
		let book  = build_codebook(&tree);

		for &b in data {
			assert!(!book[b as usize].is_empty(),
				"byte {} should have a code", b);
		}
	}

	#[test]
	fn test_higher_freq_gets_shorter_code() {
		// 'a' appears 5x, others appear fewer times'a' should be shortest
		let data = b"abracadabra"; // a=5, b=2, r=2, c=1, d=1
		let freq = count_frequencies(data);
		let tree = build_tree(&freq).unwrap();
		let book = build_codebook(&tree);

		let a_len = book[b'a' as usize].len();
		let c_len = book[b'c' as usize].len();
		let d_len = book[b'd' as usize].len();
		assert!(a_len <= c_len, "'a' should have code <= 'c'");
		assert!(a_len <= d_len, "'a' should have code <= 'd'");
	}

	#[test]
	fn test_codes_are_prefix_free() {
		let data = b"hello world";
		let freq = count_frequencies(data);
		let tree = build_tree(&freq).unwrap();
		let book = build_codebook(&tree);

		let codes: Vec<&Vec<bool>> = book.iter()
			.filter(|c| !c.is_empty())
			.collect();

		// No code should be a prefix of another
		for (i, a) in codes.iter().enumerate() {
			for (j, b) in codes.iter().enumerate() {
				if i != j && a.len() <= b.len() {
					assert!(&b[..a.len()] != a.as_slice(),
						"Code {:?} is a prefix of {:?}", a, b);
				}
			}
		}
	}
}