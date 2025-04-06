use std::io::{Read, Write};

pub const REGULAR_PACKET: &[u8] = &[
    80, 79, 83, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32,
    49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58,
    32, 42, 47, 42, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103,
    58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101, 13, 10, 85, 115, 101,
    114, 45, 65, 103, 101, 110, 116, 58, 32, 80, 121, 116, 104, 111, 110, 47, 51, 46, 49, 50, 32,
    97, 105, 111, 104, 116, 116, 112, 47, 51, 46, 49, 49, 46, 49, 54, 13, 10, 67, 111, 110, 116,
    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 13, 10, 67, 111, 110, 116, 101,
    110, 116, 45, 84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
    47, 111, 99, 116, 101, 116, 45, 115, 116, 114, 101, 97, 109, 13, 10, 13, 10, 72, 101, 108, 108,
    111, 32, 87, 111, 114, 108, 100,
];

pub const EXPECTED: &str ="POST / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nAccept: */*\r\nAccept-Encoding: gzip, deflate\r\nUser-Agent: Python/3.12 aiohttp/3.11.16\r\nContent-Length: 11\r\nContent-Type: application/octet-stream\r\n\r\nHello World";

pub struct TcpStreamMock {
    pub data: Vec<u8>,
    pub receive: Vec<u8>,
    pos: usize,
}

impl TcpStreamMock {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            receive: vec![0u8; 0],
            pos: 0,
        }
    }
}

impl Read for TcpStreamMock {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.pos >= self.data.len() {
            return Ok(0);
        }
        let bytes = &self.data[self.pos..];
        let len = bytes.len().min(buffer.len());
        buffer[..len].copy_from_slice(&bytes[..len]);
        self.pos += len;
        Ok(len)
    }
    fn read_exact(&mut self, _buffer: &mut [u8]) -> std::io::Result<()> {
        Ok(())
    }
}
impl Write for TcpStreamMock {
    fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
        Ok(1)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buffer: &[u8]) -> std::io::Result<()> {
        for c in buffer.iter() {
            self.receive.push(*c);
        }
        Ok(())
    }
}
