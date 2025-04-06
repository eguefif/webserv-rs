use crate::worker::MAX_BODY_SIZE;
use std::{error::Error, iter::Peekable};

use crate::http_error::HttpError;

#[derive(PartialEq)]
enum ChunkState {
    Header,
    Body,
    Done,
    WaitingMoreData,
}
pub struct ChunkHandler {
    pub body: Vec<u8>,
    pub leftover: Vec<u8>,
    size_str: Vec<u8>,
    size: usize,
    state: ChunkState,
}

impl ChunkHandler {
    pub fn new(leftover: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut chunk_handler = Self {
            size_str: vec![0u8; 0],
            size: 0,
            body: vec![0u8; 0],
            leftover: vec![0u8; 0],
            state: ChunkState::Header,
        };
        if leftover.len() > 0 {
            chunk_handler.parse_chunks(leftover)?;
        }
        Ok(chunk_handler)
    }

    pub fn is_body_ready(&self) -> bool {
        if self.state == ChunkState::Done {
            return true;
        }
        false
    }

    pub fn parse_chunks(&mut self, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.body.len() >= MAX_BODY_SIZE {
            return Err(Box::new(HttpError::Error413));
        }
        let mut iter = buffer.iter().peekable();
        self.reinitialize_state();
        loop {
            match self.state {
                ChunkState::Header => self.parse_header(&mut iter),
                ChunkState::Body => self.parse_body(&mut iter),
                ChunkState::Done => return Ok(()),
                ChunkState::WaitingMoreData => return Ok(()),
            }?;
        }
    }

    fn reinitialize_state(&mut self) {
        if self.state == ChunkState::WaitingMoreData {
            if self.size == 0 {
                self.state = ChunkState::Header;
            } else {
                self.state = ChunkState::Body;
            }
        }
    }

    fn parse_header<'a, I>(&mut self, iter: &mut Peekable<I>) -> Result<(), Box<dyn Error>>
    where
        I: Iterator<Item = &'a u8>,
    {
        while let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if *next == 13 && **peek == 10 {
                    self.size = parse_size(&self.size_str)?;
                    self.size_str.clear();
                    self.state = ChunkState::Body;
                    iter.next();
                    return Ok(());
                } else {
                    self.size_str.push(*next);
                }
            } else {
                self.state = ChunkState::WaitingMoreData;
                break;
            }
        }
        self.state = ChunkState::WaitingMoreData;
        Ok(())
    }

    fn parse_body<'a, I>(&mut self, iter: &mut Peekable<I>) -> Result<(), Box<dyn Error>>
    where
        I: Iterator<Item = &'a u8>,
    {
        if self.size == 0 {
            expect_cr_cn(iter)?;
            self.leftover = iter.cloned().collect::<Vec<u8>>();
            self.state = ChunkState::Done;
            return Ok(());
        }
        while self.size != 0 {
            if let Some(next) = iter.next() {
                self.body.push(*next);
            } else {
                self.state = ChunkState::WaitingMoreData;
                break;
            }
            self.size -= 1;
        }
        self.state = ChunkState::Header;
        expect_cr_cn(iter)?;
        Ok(())
    }
}

fn parse_size(size: &[u8]) -> Result<usize, Box<dyn Error>> {
    if let Ok(retval) = String::from_utf8_lossy(size).parse::<usize>() {
        Ok(retval)
    } else {
        Err(Box::new(HttpError::ErrorParsingChunkSize))
    }
}

fn expect_cr_cn<'a, I>(iter: &mut Peekable<I>) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = &'a u8>,
{
    if let None = iter.next() {
        return Err(Box::new(HttpError::Error400));
    }
    if let None = iter.next() {
        return Err(Box::new(HttpError::Error400));
    }
    Ok(())
}
