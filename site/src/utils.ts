// File Reading

/** Read a File object into a Uint8Array */
export function readFileAsBytes(file: File): Promise<Uint8Array> {
	return new Promise((resolve, reject) => {
	const reader = new FileReader();
	reader.onload = () => resolve(new Uint8Array(reader.result as ArrayBuffer));
	reader.onerror = () => reject(new Error("Failed to read file"));
	reader.readAsArrayBuffer(file);
	});
}

// Download

/** Trigger a browser download for a byte array */
export function downloadBytes(bytes: Uint8Array, filename: string): void {
	const blob = new Blob([bytes as BlobPart]);
	const url = URL.createObjectURL(blob);
	const a = document.createElement("a");

	a.href = url;
	a.download = filename;
	a.click();

	// Clean up the object URL after the download starts
	setTimeout(() => URL.revokeObjectURL(url), 1000);
}

/** Derive output filename from input filename */
export function compressedName(name: string, isFolder = false): string {
	return isFolder ? `${name}.zip.huff` : `${name}.huff`;
}

export function decompressedName(name: string): string {
	// Strip .huff extension if present
	return name.endsWith(".huff") ? name.slice(0, -5) : `${name}.decompressed`;
}

// Formatting

/** Format bytes into a human-readable string e.g. 1.23 MB */
export function formatBytes(bytes: number): string {
	if (bytes === 0) return "0 B";
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(2)} KB`;
	if (bytes < 1024 ** 3) return `${(bytes / 1024 ** 2).toFixed(2)} MB`;

	return `${(bytes / 1024 ** 3).toFixed(2)} GB`;
}

/** Format a ratio percentage, e.g. "38.2%" */
export function formatRatio(ratio: number): string {
	return `${ratio.toFixed(1)}%`;
}

// UI Helpers

export function setStatus(
	el: HTMLElement,
	message: string,
	type: "idle" | "loading" | "success" | "error"
): void {
	el.textContent = message;
	el.dataset.status = type;
}

export function showElement(el: HTMLElement): void { el.style.display = ""; }
export function hideElement(el: HTMLElement): void { el.style.display = "none"; }