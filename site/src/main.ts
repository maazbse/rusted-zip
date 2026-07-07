import { initWasm } from "./wasm";
import { initAnimations } from "./animations";
import "./style.css";
import './animations/lenis'

import { initCompressBox } from "./compress";
import { initDecompressBox } from "./decompress";

async function main(): Promise<void> {
	const appEl = document.getElementById("app")!;

	try {
		await initWasm();
		appEl.dataset.ready = "true";

		// Initialize compression and decompression box logic
		initCompressBox();
		initDecompressBox();
		
		requestAnimationFrame(() => {
			initAnimations();
		});

	} catch (err) {
		appEl.innerHTML = `
			<div class="init-error">
				Failed to load WASM module: ${err}<br/>
				Please refresh the page or check your browser supports WebAssembly.
			</div>
		`;
		return;
	}

	// ── Smooth scroll for anchor links ──
	document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]').forEach((anchor) => {
		anchor.addEventListener("click", (e) => {
			e.preventDefault();
			const targetId = anchor.getAttribute("href");
			if (!targetId) return;
			const target = document.querySelector(targetId);
			if (target) {
				target.scrollIntoView({ behavior: "smooth" });
			}
		});
	});
}

main();