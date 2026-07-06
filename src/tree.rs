use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Node Definition

/// A single node in the Huffman tree.
/// Either a Leaf (holds a byte value) or an Internal node (holds two children).
#[derive(Debug)]
pub enum HuffmanNode {
	Leaf {
		byte: u8,
		frequency: u64,
	},
	Internal {
		frequency: u64,
		left: Box<HuffmanNode>,  // Box = heap-allocated, allows recursive types
		right: Box<HuffmanNode>,
	},
}

impl HuffmanNode {
	pub fn frequency(&self) -> u64 {
		match self {
			HuffmanNode::Leaf { frequency, .. } => *frequency,
			HuffmanNode::Internal { frequency, .. } => *frequency,
		}
	}
}

// Ordering for the Min-Heap
// Rust's BinaryHeap is a MAX-heap by default.
// Wrapping nodes in HeapEntry and reverse the ordering to get a MIN-heap.

struct HeapEntry(Box<HuffmanNode>);

impl PartialEq for HeapEntry {
	fn eq(&self, other: &Self) -> bool {
		self.0.frequency() == other.0.frequency()
	}
}
impl Eq for HeapEntry {}

// Reverse the natural order so that LOWER frequency = HIGHER priority
impl Ord for HeapEntry {
	fn cmp(&self, other: &Self) -> Ordering {
		other.0.frequency().cmp(&self.0.frequency())
	}
}
impl PartialOrd for HeapEntry {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// Tree Construction

/// Builds a Huffman tree from a frequency table.
/// Returns None if the input was empty (all frequencies are zero).
pub fn build_tree(frequencies: &[u64; 256]) -> Option<Box<HuffmanNode>> {
	let mut heap = BinaryHeap::new();

	// Step 1: Push one Leaf per byte that actually appears
	for (byte, &freq) in frequencies.iter().enumerate() {
		if freq > 0 {
			heap.push(HeapEntry(Box::new(HuffmanNode::Leaf {
				byte: byte as u8,
				frequency: freq,
			})));
		}
	}

	// Edge case: file was empty
	if heap.is_empty() {
		return None;
	}

	// Edge case: only one unique byte — wrap it in an internal node
	// so the tree always has at least one left/right branch to traverse
	if heap.len() == 1 {
		let only = heap.pop().unwrap().0;
		let freq = only.frequency();
		return Some(Box::new(HuffmanNode::Internal {
			frequency: freq,
			left: only,
			right: Box::new(HuffmanNode::Leaf { byte: 0, frequency: 0 }),
		}));
	}

	// Step 2: Merge the two lowest-frequency nodes repeatedly
	while heap.len() > 1 {
		let left = heap.pop().unwrap().0;  // lowest freq
		let right = heap.pop().unwrap().0;  // second lowest

		let merged = Box::new(HuffmanNode::Internal {
			frequency: left.frequency() + right.frequency(),
			left,
			right,
		});
		heap.push(HeapEntry(merged));
	}

	// Step 3: The last remaining node is the root
	Some(heap.pop().unwrap().0)
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::frequency::count_frequencies;

	#[test]
	fn test_empty_gives_none() {
		let freq = count_frequencies(&[]);
		assert!(build_tree(&freq).is_none());
	}

	#[test]
	fn test_single_unique_byte() {
		let freq = count_frequencies(&[7u8, 7, 7]);
		let tree = build_tree(&freq).unwrap();
		// Root should be Internal wrapping a Leaf with byte=7
		match *tree {
			HuffmanNode::Internal { left, .. } => match *left {
				HuffmanNode::Leaf { byte, frequency } => {
					assert_eq!(byte, 7);
					assert_eq!(frequency, 3);
				}
				_ => panic!("Expected leaf on left"),
			},
			_ => panic!("Expected internal root"),
		}
	}

	#[test]
	fn test_root_frequency_equals_total_bytes() {
		let data = b"hello world";
		let freq = count_frequencies(data);
		let tree = build_tree(&freq).unwrap();
		assert_eq!(tree.frequency(), data.len() as u64);
	}
}