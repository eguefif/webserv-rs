use flate2::read::DeflateDecoder;
use flate2::read::GzDecoder;
use std::io::Read;

pub enum Encoding {
    Gzip,
    Deflate,
}

pub fn uncompress(data: Vec<u8>, encoding: Encoding) -> Vec<u8> {
    match encoding {
        Encoding::Gzip => gzip(data),
        Encoding::Deflate => deflate(data),
    }
}

fn gzip(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = GzDecoder::new(&data[..]);
    let mut retval = vec![0u8; 0];
    let _ = decoder.read(&mut retval);
    retval
}

fn deflate(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(&data[..]);
    let mut retval = vec![0u8; 0];
    let _ = decoder.read(&mut retval);
    retval
}
