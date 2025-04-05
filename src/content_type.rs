use std::fmt;

pub enum ContentType {
    TextHtml,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ContentType::TextHtml => "text/html; charset=utf-8",
        };
        write!(f, "{}", value)
    }
}
