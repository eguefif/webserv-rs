use std::iter::Peekable;

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
    pub fn new(leftover: &[u8]) -> Self {
        let mut chunk_handler = Self {
            size_str: vec![0u8; 0],
            size: 0,
            body: vec![0u8; 0],
            leftover: vec![0u8; 0],
            state: ChunkState::Header,
        };
        if leftover.len() > 0 {
            chunk_handler.parse_chunks(leftover);
        }
        chunk_handler
    }

    pub fn is_body_ready(&self) -> bool {
        if self.state == ChunkState::Done {
            return true;
        }
        false
    }

    pub fn parse_chunks(&mut self, buffer: &[u8]) {
        let mut iter = buffer.iter().peekable();
        self.reinitialize_state();
        loop {
            match self.state {
                ChunkState::Header => self.parse_header(&mut iter),
                ChunkState::Body => self.parse_body(&mut iter),
                ChunkState::Done => return,
                ChunkState::WaitingMoreData => return,
            }
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

    fn parse_header<'a, I>(&mut self, iter: &mut Peekable<I>)
    where
        I: Iterator<Item = &'a u8>,
    {
        while let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if *next == 13 && **peek == 10 {
                    self.size = parse_size(&self.size_str);
                    self.size_str.clear();
                    self.state = ChunkState::Body;
                    iter.next();
                    return;
                } else {
                    self.size_str.push(*next);
                }
            } else {
                self.state = ChunkState::WaitingMoreData;
                break;
            }
        }
        self.state = ChunkState::WaitingMoreData;
    }

    fn parse_body<'a, I>(&mut self, iter: &mut Peekable<I>)
    where
        I: Iterator<Item = &'a u8>,
    {
        if self.size == 0 {
            iter.next();
            iter.next();
            self.leftover = iter.cloned().collect::<Vec<u8>>();
            self.state = ChunkState::Done;
            return;
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
        iter.next();
        iter.next();
    }
}

fn parse_size(size: &[u8]) -> usize {
    String::from_utf8_lossy(size).parse::<usize>().unwrap()
}
