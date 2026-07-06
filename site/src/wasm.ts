import init, { compress, decompress, WasmResult } from "../../pkg/rusted_zip.js";

// Initialisation
let initialised = false;

// Load and initialise the WASM module.
export async function initWasm(): Promise<void> {
	if (initialised) return;
	await init();
	initialised = true;
}

// Struct for Typed Result
export interface HuffmanResult {
	success: boolean;
	data: Uint8Array;
	error: string;
	originalSize: number;
	compressedSize: number;
	ratio: number;   // compressed / original * 100
	savings: number;   // 100 - ratio
}

function toResult(r: WasmResult): HuffmanResult {
	return {
		success: r.success(),
		data: r.data(),
		error: r.error(),
		originalSize: r.original_size(),
		compressedSize: r.compressed_size(),
		ratio: r.ratio(),
		savings: r.savings(),
	};
}

// Public API

// Compress raw bytes. Returns a typed result object.
export function huffmanCompress(data: Uint8Array): HuffmanResult {
	return toResult(compress(data));
}

// Decompress a .huff file. Returns a typed result object.
export function huffmanDecompress(data: Uint8Array): HuffmanResult {
	return toResult(decompress(data));
}