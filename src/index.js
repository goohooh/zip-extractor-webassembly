async function readFile(file) {
    const arrayBuffer = await new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.addEventListener("loadend", () => resolve(reader.result));
        reader.addEventListener("error", reject);
        reader.readAsArrayBuffer(file);
    });
    return new Uint8Array(arrayBuffer);
}

document.querySelector(".js-input")
    .addEventListener("change", async (e) => {
        const wasm = await import("./crate/pkg");
        const file = e.target.files[0];
        if (!file) {
            return false;
        }
        e.target.toggleAttribute("disabled", true);
        const buffer = await readFile(file);
        console.log(wasm.getFilenameList(buffer));
    });