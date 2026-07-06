import { initWasm } from "./wasm";
import { initCompressBox } from "./compress";
import { initDecompressBox } from "./decompress";

async function main(): Promise<void> {
	const appEl = document.getElementById("app")!;

	try {
		await initWasm();
		appEl.dataset.ready = "true";
		initCompressBox();
		initDecompressBox();
	} catch (err) {
		appEl.innerHTML = `
			<div class="init-error">
				Failed to load WASM module: ${err}<br/>
			</div>
		`;
	}
}

main();