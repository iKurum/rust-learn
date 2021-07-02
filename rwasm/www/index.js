import * as wasm from "rwasm";

window.btn = function () {
  document.getElementById('files').click();
};

window.fileImport = function () {
  //获取读取文件的 File 对象
  var selectedFile = document.getElementById('files').files[0];
  var reader = new FileReader(); // 这是核心, 读取操作就是由它完成.
  reader.readAsArrayBuffer(selectedFile); // 读取文件的内容,也可以读取文件的URL
  reader.onload = function () {
    var uint8Array = new Uint8Array(this.result);

    console.log(uint8Array);
    wasm.grayscale(uint8Array);
  }
};

