import { unzip, Zip, ZipDeflate, type ZipInputFile } from "fflate";

export interface ArchiveEntry {
  path: string;
  bytes: Uint8Array;
}

/**
 * Pack files into a ZIP archive one at a time using fflate's streaming Zip
 * API. Only one file's bytes live in memory at a time, so large folders no
 * longer cause an OOM crash.
 *
 * @param files       Raw File objects from the browser.
 * @param onProgress  Optional callback: (filesProcessed, totalFiles) => void.
 */
export function pack(
  files: File[],
  onProgress?: (done: number, total: number) => void
): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const chunks: Uint8Array[] = [];

    const zipper = new Zip((err, chunk, final) => {
      if (err) { reject(err); return; }
      chunks.push(chunk);
      if (final) {
        // Merge all streamed chunks into one buffer.
        let totalLen = 0;
        for (const c of chunks) totalLen += c.length;
        const out = new Uint8Array(totalLen);
        let offset = 0;
        for (const c of chunks) { out.set(c, offset); offset += c.length; }
        resolve(out);
      }
    });

    // Feed files sequentially so only one ArrayBuffer exists at a time.
    (async () => {
      try {
        for (let i = 0; i < files.length; i++) {
          const file = files[i];
          const path = file.webkitRelativePath || file.name;
          const bytes = new Uint8Array(await file.arrayBuffer());

          const entry = new ZipDeflate(path, { level: 0 }) as ZipInputFile;
          zipper.add(entry);
          (entry as ZipDeflate).push(bytes, true);   // true = last (and only) chunk

          if (onProgress) onProgress(i + 1, files.length);

          // Yield to the event loop between files to keep the browser alive.
          await tick();
        }
        zipper.end();
      } catch (err) {
        reject(err);
      }
    })();
  });
}

/** Read a single file's bytes. Kept for symmetry; used by the file path. */
export function readFileAsBytes(file: File): Promise<Uint8Array> {
  return file.arrayBuffer().then((buf) => new Uint8Array(buf));
}

export function unpack(zipBytes: Uint8Array): Promise<ArchiveEntry[]> {
  return new Promise((resolve, reject) => {
    unzip(zipBytes, (err, files) => {
      if (err) { reject(err); return; }
      const entries: ArchiveEntry[] = Object.entries(files).map(
        ([path, bytes]) => ({ path, bytes })
      );
      resolve(entries);
    });
  });
}

function tick(): Promise<void> {
  return new Promise((r) => setTimeout(r, 0));
}