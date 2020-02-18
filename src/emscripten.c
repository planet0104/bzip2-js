#include "D:\emsdk\emsdk\upstream\emscripten\system\include\emscripten\emscripten.h"

//EM_JS用法 参考 https://emscripten.org/docs/api_reference/emscripten.h.html

EM_JS(void, _js_console_log, (const char* str), {
    var msg = UTF8ToString(str);
    console.log("Bzip2", msg);
});

EM_JS(void, _js_console_error, (const char* str), {
    var msg = UTF8ToString(str);
    console.error("Bzip2", msg);
});

EM_JS(void, _js_set_module_field, (const char* key, const char* value), {
  Module[UTF8ToString(key)] = UTF8ToString(value);
});

EM_JS(void, _js_set_module_field_json, (const char* key, const char* value), {
  Module[UTF8ToString(key)] = JSON.parse(UTF8ToString(value));
});

EM_JS(void, _js_delete_module_field, (const char* key), {
  Module[UTF8ToString(key)] = null;
});

EM_JS(void, _js_init, (), {

  function allocTypedArrayBuffer(array) {
    var buf = Module._malloc(array.length * array.BYTES_PER_ELEMENT);
    Module.HEAPU8.set(array, buf);
    // console.log('allocBuffer 长度:' + array.length+"指针:"+buf);
    return buf;
    //这里不调用free，由rust代码中from_raw自动释放
    //getApp().Module._free(buf);
  };
  Module.Bzip2 = {
    compress: function(array){
      Module["_compress"](allocTypedArrayBuffer(array), array.length);
      var result = Module["compressResult"];
      var data = new Uint8Array(Module.HEAPU8.subarray(result.dataPtr, result.dataPtr + result.dataLen).slice(0));
      //rust中使用 mem::forget传递过来的指针，要在asmjs中手动释放
      Module._free(result.dataPtr);
      return data;
    },
    decompress_to_utf8string: function(array){
      Module["_decompress_to_utf8string"](allocTypedArrayBuffer(array), array.length);
      return Module["decompressResult"];
    },
    decompress: function(array){
      Module["_decompress"](allocTypedArrayBuffer(array), array.length);
      var result = Module["decompressResult"];
      var data = new Uint8Array(Module.HEAPU8.subarray(result.dataPtr, result.dataPtr + result.dataLen).slice(0));
      Module._free(result.dataPtr);
      return data;
    }
  };
});

void js_init(){
    _js_init();
}

void js_set_module_field(const char* key, const char* value){
    _js_set_module_field(key, value);
}

void js_set_module_field_json(const char* key, const char* value){
    _js_set_module_field_json(key, value);
}

void js_console_log(const char* msg){
    _js_console_log(msg);
}

void js_console_error(const char* msg){
    _js_console_error(msg);
}

void js_delete_module_field(const char* key){
    _js_delete_module_field(key);
}