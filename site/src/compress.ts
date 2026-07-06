import { huffmanCompress } from "./wasm";
import { pack } from "./archive";
import { readFileAsBytes, downloadBytes,
         compressedName, formatBytes, setStatus } from "./utils";

export function initCompressBox(): void {
  const dropzone  = document.getElementById("compress-dropzone")!  as HTMLElement;
  const fileInput = document.getElementById("compress-file")!       as HTMLInputElement;
  const folderInput = document.getElementById("compress-folder")!   as HTMLInputElement;
  const fileName  = document.getElementById("compress-filename")!   as HTMLElement;
  const runBtn    = document.getElementById("compress-run")!         as HTMLButtonElement;
  const status    = document.getElementById("compress-status")!      as HTMLElement;
  const output    = document.getElementById("compress-output")!      as HTMLElement;

  document.getElementById("compress-pick-file")!
    .addEventListener("click", () => fileInput.click());
  document.getElementById("compress-pick-folder")!
    .addEventListener("click", () => folderInput.click());

  type Selection =
    | { kind: "file";   file: File;    outputName: string }
    | { kind: "folder"; files: File[]; folderName: string; outputName: string }
    | null;

  let selection: Selection = null;
  let resultBytes: Uint8Array | null = null;

  // ── File picker ──────────────────────────────────────────────────────────
  fileInput.addEventListener("change", () => {
    const file = fileInput.files?.[0];
    if (!file) return;
    selection = { kind: "file", file, outputName: compressedName(file.name) };
    onPicked(`${file.name} (${formatBytes(file.size)})`);
  });

  // ── Folder picker ────────────────────────────────────────────────────────
  folderInput.addEventListener("change", () => {
    const files = Array.from(folderInput.files ?? []);
    if (files.length === 0) return;
    const folderName = files[0].webkitRelativePath.split("/")[0];
    selection = { kind: "folder", files, folderName, outputName: compressedName(folderName, true) };
    const totalSize = files.reduce((s, f) => s + f.size, 0);
    onPicked(`📁 ${folderName} (${files.length} files, ${formatBytes(totalSize)})`);
  });

  // ── Drag-and-drop ────────────────────────────────────────────────────────
  dropzone.addEventListener("dragover", (e) => {
    e.preventDefault();
    dropzone.dataset.dragging = "true";
  });
  dropzone.addEventListener("dragleave", () => delete dropzone.dataset.dragging);
  dropzone.addEventListener("drop", async (e) => {
    e.preventDefault();
    delete dropzone.dataset.dragging;
    const items = Array.from(e.dataTransfer?.items ?? []);
    const entry = items[0]?.webkitGetAsEntry?.();
    if (!entry) return;

    if (entry.isFile) {
      const file = e.dataTransfer!.files[0];
      selection = { kind: "file", file, outputName: compressedName(file.name) };
      onPicked(`${file.name} (${formatBytes(file.size)})`);
    } else if (entry.isDirectory) {
      setStatus(status, "Reading folder...", "loading");
      const files = await readDroppedFolder(entry as FileSystemDirectoryEntry);
      if (files.length === 0) return;
      const folderName = entry.name;
      selection = { kind: "folder", files, folderName, outputName: compressedName(folderName, true) };
      const totalSize = files.reduce((s, f) => s + f.size, 0);
      onPicked(`📁 ${folderName} (${files.length} files, ${formatBytes(totalSize)})`);
    }
  });

  function onPicked(label: string): void {
    resultBytes = null;
    fileName.textContent = label;
    output.innerHTML = "";
    setStatus(status, "Ready — click Compress", "idle");
    runBtn.disabled = false;
  }

  // ── Compress button ───────────────────────────────────────────────────────
  runBtn.addEventListener("click", async () => {
    if (!selection) return;
    runBtn.disabled = true;
    output.innerHTML = "";

    try {
      let inputBytes: Uint8Array;
      let inputSize: number;

      if (selection.kind === "file") {
        setStatus(status, "Reading file...", "loading");
        await tick();
        inputBytes = await readFileAsBytes(selection.file);
        inputSize  = inputBytes.length;

      } else {
        // Folder path: stream files through the ZIP packer one at a time.
        const totalFiles = selection.files.length;
        inputSize = selection.files.reduce((s, f) => s + f.size, 0);

        setStatus(status, `Packing 0 / ${totalFiles} files…`, "loading");
        await tick();

        inputBytes = await pack(selection.files, (done, total) => {
          // Update status every 10 files to avoid thrashing the DOM.
          if (done % 10 === 0 || done === total) {
            setStatus(status, `Packing ${done} / ${total} files…`, "loading");
          }
        });
      }

      setStatus(status, "Compressing…", "loading");
      await tick();

      const result = huffmanCompress(inputBytes);

      if (!result.success) {
        setStatus(status, `Error: ${result.error}`, "error");
        runBtn.disabled = false;
        return;
      }

      resultBytes = result.data;
      output.innerHTML = `
        <dl class="stats">
          <dt>Original</dt>   <dd>${formatBytes(inputSize)}</dd>
          <dt>Compressed</dt> <dd>${formatBytes(result.compressedSize)}</dd>
          <dt>Ratio</dt>      <dd>${result.ratio.toFixed(1)}%</dd>
          <dt>Saved</dt>      <dd>${result.savings.toFixed(1)}%</dd>
        </dl>
        <button class="btn btn-secondary" id="compress-download">
          Download .huff
        </button>
      `;

      document.getElementById("compress-download")!
        .addEventListener("click", () => {
          if (resultBytes && selection) {
            downloadBytes(resultBytes, selection.outputName);
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

function tick(): Promise<void> {
  return new Promise((r) => setTimeout(r, 0));
}

// ── Drag-and-drop folder helpers ────────────────────────────────────────────

async function readDroppedFolder(
  dir: FileSystemDirectoryEntry,
  prefix = ""
): Promise<File[]> {
  const results: File[] = [];
  const entries = await readDirEntries(dir);
  for (const entry of entries) {
    if (entry.isFile) {
      const file = await getFile(entry as FileSystemFileEntry);
      Object.defineProperty(file, "webkitRelativePath", {
        value: `${prefix}${dir.name}/${file.name}`,
      });
      results.push(file);
    } else if (entry.isDirectory) {
      const sub = await readDroppedFolder(
        entry as FileSystemDirectoryEntry,
        `${prefix}${dir.name}/`
      );
      results.push(...sub);
    }
  }
  return results;
}

function readDirEntries(dir: FileSystemDirectoryEntry): Promise<FileSystemEntry[]> {
  return new Promise((resolve, reject) => {
    const reader  = dir.createReader();
    const results: FileSystemEntry[] = [];
    function read(): void {
      reader.readEntries((batch) => {
        if (batch.length === 0) resolve(results);
        else { results.push(...batch); read(); }
      }, reject);
    }
    read();
  });
}

function getFile(entry: FileSystemFileEntry): Promise<File> {
  return new Promise((resolve, reject) => entry.file(resolve, reject));
}