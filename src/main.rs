use std::cell::RefCell;
use std::time::Duration;
mod bzip2;
mod emscripten;
use emscripten::*;

//emscripten版本1.39.0
//如果编译完报错：TypeError: Right-hand side of 'instanceof' is not an object，说明是内存不够用！
//https://emscripten.org/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html#calling-javascript-from-c-c

/// 压缩
#[no_mangle]
pub fn compress(buffer: *mut u8, buf_len: i32) {
    let data: Vec<u8> = unsafe { Vec::from_raw_parts(buffer, buf_len as usize, buf_len as usize) };
    let result = bzip2::compress(data);
    //将指针存储到Module中，在Module.Bzip2的compress js方法中获取
    let len = result.len();
    let ptr = result.as_ptr();
    std::mem::forget(result);
    set_module_field_json(
        "compressResult",
        &format!(
            r#"{{
        "dataLen": {},
        "dataPtr": {}
    }}"#,
            len, ptr as i64
        ),
    );
}

//解压缩
#[no_mangle]
pub fn decompress(buffer: *mut u8, buf_len: i32) {
    let data: Vec<u8> = unsafe { Vec::from_raw_parts(buffer, buf_len as usize, buf_len as usize) };
    let result = bzip2::decompress(data);
    //将指针存储到Module中，在Module.Bzip2的decompress js方法中获取
    let len = result.len();
    let ptr = result.as_ptr();
    std::mem::forget(result);
    set_module_field_json(
        "decompressResult",
        &format!(
            r#"{{
        "dataLen": {},
        "dataPtr": {}
    }}"#,
            len, ptr as i64
        ),
    );
}

#[no_mangle]
pub fn decompress_to_utf8string(buffer: *mut u8, buf_len: i32) {
    let data: Vec<u8> = unsafe { Vec::from_raw_parts(buffer, buf_len as usize, buf_len as usize) };
    let result = bzip2::decompress(data);
    match String::from_utf8(result) {
        Ok(result) => {
            //将结果存储到Module中，在Module.Bzip2的decompress_to_utf8string js方法中获取
            set_module_field("decompressResult", &result);
        }
        Err(err) => {
            let err_str = format!("{:?}", err);
            console_error(&err_str);
        }
    }
}

fn main() {
    init();
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
