use flate2::read::DeflateDecoder;
use flate2::read::GzDecoder;
use std::error::Error;
use std::io::Read;

pub enum Encoding {
    Gzip,
    Deflate,
}

pub fn uncompress(data: &[u8], encoding: Encoding) -> Result<Vec<u8>, Box<dyn Error>> {
    match encoding {
        Encoding::Gzip => gzip(data),
        Encoding::Deflate => deflate(data),
    }
}

fn gzip(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decoder = GzDecoder::new(&data[..]);
    let mut retval = Vec::new();
    decoder.read_to_end(&mut retval)?;
    Ok(retval)
}

fn deflate(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decoder = DeflateDecoder::new(&data[..]);
    let mut retval = Vec::new();
    decoder.read_to_end(&mut retval)?;
    Ok(retval)
}
