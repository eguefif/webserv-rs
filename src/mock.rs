use std::io::{Read, Write};

pub const REGULAR_PACKET: &[&[u8]] = &[&[
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
]];

pub const CHUNKED: &[&[u8]] = &[
    &[
        80, 79, 83, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58,
        32, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48, 13, 10, 65, 99, 99, 101, 112,
        116, 58, 32, 42, 47, 42, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
        110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101, 13, 10, 85,
        115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 80, 121, 116, 104, 111, 110, 47, 51, 46,
        49, 50, 32, 97, 105, 111, 104, 116, 116, 112, 47, 51, 46, 49, 49, 46, 49, 54, 13, 10, 67,
        111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99,
        97, 116, 105, 111, 110, 47, 111, 99, 116, 101, 116, 45, 115, 116, 114, 101, 97, 109, 13,
        10, 84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103, 58, 32,
        99, 104, 117, 110, 107, 101, 100, 13, 10, 13, 10,
    ],
    &[53, 13, 10, 72, 101, 108, 108, 111, 13, 10],
    &[53, 13, 10, 87, 111, 114, 108, 100, 13, 10],
    &[52, 13, 10, 102, 114, 111, 109, 13, 10],
    &[51, 13, 10, 116, 104, 101, 13, 10],
    &[51, 13, 10, 115, 107, 121, 13, 10],
    &[48, 13, 10, 13, 10],
];

pub const EXPECTED: &str ="POST / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nAccept: */*\r\nAccept-Encoding: gzip, deflate\r\nUser-Agent: Python/3.12 aiohttp/3.11.16\r\nContent-Length: 11\r\nContent-Type: application/octet-stream\r\n\r\nHello World";

pub struct TcpStreamMock {
    pub data: Vec<Vec<u8>>,
    pub receive: Vec<u8>,
    current: Vec<u8>,
    pos: usize,
}

impl TcpStreamMock {
    pub fn new(request_bytes: &[&[u8]]) -> Self {
        let mut data = Vec::new();
        for entry in request_bytes.into_iter() {
            data.push(entry.to_vec())
        }
        data.reverse();
        let current = data.pop().expect("There are no test data in TcpStreamMock");
        Self {
            data,
            receive: vec![0u8; 0],
            pos: 0,
            current,
        }
    }
}

impl Read for TcpStreamMock {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.pos >= self.current.len() {
            if let Some(data) = self.data.pop() {
                self.pos = 0;
                self.current = data;
            } else {
                return Ok(0);
            }
        }
        let bytes = &self.current[self.pos..];
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
