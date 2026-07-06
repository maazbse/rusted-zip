use wasm_bindgen::prelude::*;

pub mod bits;
pub mod codebook;
pub mod codec;
pub mod frequency;
pub mod tree;

// WASM Glue

/// Called once by JS to pipe Rust panics into browser console.error()
/// instead of crashing with no output in debug builds
#[wasm_bindgen(start)]
pub fn init() {
	#[cfg(debug_assertions)]
	console_error_panic_hook();
}

fn console_error_panic_hook() {
	use std::panic;
	panic::set_hook(Box::new(|info| {
		web_sys::console::error_1(&format!("Rust panic: {}", info).into());
	}));
}

// Exported Types

/// The result of a compress or decompress operation.
/// Returned as a JS object with { success, data, error, stats }.
#[wasm_bindgen]
pub struct WasmResult {
	success: bool,
	data: Vec<u8>,
	error: String,
	original_size: usize,
	compressed_size: usize,
}

#[wasm_bindgen]
impl WasmResult {
	pub fn success(&self) -> bool { self.success }

	pub fn data(&self) -> Vec<u8> { self.data.clone() }

	pub fn error(&self) -> String { self.error.clone() }

	pub fn original_size(&self) -> usize { self.original_size }

	pub fn compressed_size(&self) -> usize { self.compressed_size }

	pub fn ratio(&self) -> f64 {
		if self.original_size == 0 { return 0.0; }
		(self.compressed_size as f64 / self.original_size as f64) * 100.0
	}

	pub fn savings(&self) -> f64 {
		100.0 - self.ratio()
	}
}

// Exported Functions

/// Compress raw bytes using Huffman encoding.
/// Call from JS: const result = compress(uint8Array);
#[wasm_bindgen]
pub fn compress(data: &[u8]) -> WasmResult {
	match codec::compress(data) {
		Ok(compressed) => {
			let original_size = data.len();
			let compressed_size = compressed.len();
			WasmResult {
				success: true,
				data: compressed,
				error: String::new(),
				original_size,
				compressed_size,
			}
		}
		Err(e) => WasmResult {
			success: false,
			data: Vec::new(),
			error: e,
			original_size: 0,
			compressed_size: 0,
		},
	}
}

/// Decompress a .huff file back to its original bytes.
/// Call from JS: const result = decompress(uint8Array);
#[wasm_bindgen]
pub fn decompress(data: &[u8]) -> WasmResult {
	let original_size = data.len();
	match codec::decompress(data) {
		Ok(decompressed) => WasmResult {
			success: true,
			compressed_size: original_size,
			original_size: decompressed.len(),
			data: decompressed,
			error: String::new(),
		},
		Err(e) => WasmResult {
			success: false,
			data: Vec::new(),
			error: e,
			original_size: 0,
			compressed_size: 0,
		},
	}
}