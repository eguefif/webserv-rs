pub struct ChunkHandler {
    current_chunk: Vec<u8>,
}

impl ChunkHandler {
    pub fn new() -> Self {
        Self {
            current_chunk: vec![0u8; 1024],
        }
    }
}
