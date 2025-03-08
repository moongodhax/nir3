let t0 = performance.now();

import initSync, { md5 } from "./pkg/md5module.js";

const fileInput = document.getElementById('file-input');

fileInput.addEventListener("change", (event) => {
	const reader = new FileReader();
	reader.onload = function(e) {
		const bytes = new Uint8Array(e.target.result);

		t0 = performance.now();
		const res = md5(bytes);
		t1 = performance.now();

		document.getElementById('result').innerHTML =`result ${t1 - t0} ${res}`;
	}
	reader.readAsArrayBuffer(event.target.files[0]);
});

initSync();
let t1 = performance.now();
document.getElementById('ready').innerHTML = `ready ${t1 - t0}`;