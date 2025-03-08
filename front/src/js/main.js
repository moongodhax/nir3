import initSync, { md5 } from "./pkg/md5module.js";

const fileInput = document.getElementById('file-input');

fileInput.addEventListener("change", (event) => {
	const files = event.target.files;
	if (files.length) {
		const filename = files[0].name;

		const resultDiv = document.getElementById('result');
		resultDiv.innerHTML = '';

		const reader = new FileReader();

		reader.onload = async function(e) {
			// var buffer = e.target.result;
			// var md5res = md5(new Uint8Array(buffer));
			// resultDiv.innerHTML = `md5(${filename}) = ${md5res}`;

			const bytes = new Uint8Array(e.target.result);
			const result = md5(bytes);
			resultDiv.innerHTML = `md5(${filename}) = ${result}`;
		}

		reader.readAsArrayBuffer(files[0]);
	}
});

initSync();