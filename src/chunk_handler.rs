pub fn parse_chunks(buffer: &[u8]) -> Vec<u8> {
    let mut body: Vec<u8> = Vec::new();

    let mut size_str: Vec<u8> = Vec::new();
    let mut iter = buffer.iter().peekable();
    loop {
        let mut size = 0;
        while let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if *next == 13 && **peek == 10 {
                    size = parse_size(&size_str);
                    size_str.clear();
                    break;
                } else {
                    size_str.push(*next);
                }
            } else {
                break;
            }
        }
        if size == 0 {
            println!("No frame");
            break;
        }
        iter.next();
        while size != 0 {
            if let Some(next) = iter.next() {
                body.push(*next);
            } else {
                break;
            }
            size -= 1;
        }
        iter.next();
        iter.next();
    }
    body
}

fn parse_size(size: &[u8]) -> usize {
    println!("size: {:?}", size);
    String::from_utf8_lossy(size).parse::<usize>().unwrap()
}
