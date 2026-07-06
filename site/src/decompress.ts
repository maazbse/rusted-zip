import { huffmanDecompress } from "./wasm";
import { readFileAsBytes, downloadBytes,
				 decompressedName, formatBytes, setStatus } from "./utils";

export function initDecompressBox(): void {
	// Element refs
	const dropzone = document.getElementById("decompress-dropzone")! as HTMLElement;
	const fileInput = document.getElementById("decompress-file")! as HTMLInputElement;
	const fileName = document.getElementById("decompress-filename")! as HTMLElement;
	const runBtn = document.getElementById("decompress-run")! as HTMLButtonElement;
	const status = document.getElementById("decompress-status")! as HTMLElement;
	const output = document.getElementById("decompress-output")! as HTMLElement;

	// State
	let selectedFile: File | null = null;
	let resultBytes: Uint8Array | null = null;

	// File selection
	dropzone.addEventListener("click", () => fileInput.click());

	fileInput.addEventListener("change", () => {
		const file = fileInput.files?.[0];
		if (file) setFile(file);
	});

	// Drag and drop
	dropzone.addEventListener("dragover", (e) => {
		e.preventDefault();
		dropzone.dataset.dragging = "true";
	});
	dropzone.addEventListener("dragleave", () => delete dropzone.dataset.dragging);
	dropzone.addEventListener("drop", (e) => {
		e.preventDefault();
		delete dropzone.dataset.dragging;
		const file = e.dataTransfer?.files[0];
		if (file) setFile(file);
	});

	function setFile(file: File): void {
		selectedFile = file;
		resultBytes = null;
		fileName.textContent = `${file.name} (${formatBytes(file.size)})`;
		output.innerHTML = "";
		setStatus(status, "Ready — click Decompress", "idle");
		runBtn.disabled = false;
	}

	// Decompress
	runBtn.addEventListener("click", async () => {
		if (!selectedFile) return;

		runBtn.disabled = true;
		output.innerHTML = "";
		setStatus(status, "Reading file...", "loading");

		try {
			const bytes = await readFileAsBytes(selectedFile);
			setStatus(status, "Decompressing...", "loading");
			await new Promise(r => setTimeout(r, 0));

			const result = huffmanDecompress(bytes);

			if (!result.success) {
				setStatus(status, `Error: ${result.error}`, "error");
				runBtn.disabled = false;
				return;
			}

			resultBytes = result.data;

			// Inject stats + download button
			output.innerHTML = `
				<dl class="stats">
					<dt>Compressed</dt> <dd>${formatBytes(result.compressedSize)}</dd>
					<dt>Decompressed</dt> <dd>${formatBytes(result.originalSize)}</dd>
				</dl>
				<button class="btn btn-secondary" id="decompress-download">
					Download file
				</button>
			`;

			document.getElementById("decompress-download")!
				.addEventListener("click", () => {
					if (resultBytes && selectedFile) {
						downloadBytes(resultBytes, decompressedName(selectedFile.name));
					}
				});

			setStatus(status, "Done!", "success");
		} catch (err) {
			setStatus(status, `Error: ${err}`, "error");
		} finally {
			runBtn.disabled = false;
		}
	});
}