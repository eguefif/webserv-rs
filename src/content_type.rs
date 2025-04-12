use std::fmt;

pub enum ContentType {
    Text,
    TextHtml,
    JS,
    CSS,
    Icon,
    Image(String),
    Json,
    SVG,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::TextHtml => write!(f, "text/html; charset=utf-8"),
            ContentType::Text => write!(f, "text/plain; charset=utf-8"),
            ContentType::JS => write!(f, "text/javascript; charset=utf-8"),
            ContentType::CSS => write!(f, "text/css; charset=utf-8"),
            ContentType::Icon => write!(f, "image/x-icon"),
            ContentType::Json => write!(f, "application/json"),
            ContentType::SVG => write!(f, "application/svg+xml"),
            ContentType::Image(image_type) => write!(f, "image/{}", image_type),
        }
    }
}
