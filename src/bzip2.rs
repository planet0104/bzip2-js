use super::emscripten::console_error;
use std::io::prelude::*;
use bzip2::Compression;
use bzip2::write::{BzEncoder, BzDecoder};

///压缩字节
pub fn compress(data: Vec<u8>) -> Vec<u8> {
    let mut zip = BzEncoder::new(vec![], Compression::Best);
    if let Err(err) = zip.write_all(&data){
        let err_str = format!("{:?}", err);
        console_error(&err_str);
        return vec![];
    }
    match zip.finish(){
        Ok(result) => result,
        Err(err) => {
            let err_str = format!("{:?}", err);
            console_error(&err_str);
            return vec![];
        }
    }
}

///解压缩到字节
pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut decompressor = BzDecoder::new(vec![]);

    if let Err(err) = decompressor.write_all(&data){
        let err_str = format!("{:?}", err);
        console_error(&err_str);
        return vec![];
    }
    match decompressor.finish(){
        Ok(result) => result,
        Err(err) => {
            let err_str = format!("{:?}", err);
            console_error(&err_str);
            return vec![];
        }
    }
}
