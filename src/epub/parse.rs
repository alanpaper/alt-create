
use encoding::{Encoding, DecoderTrap};
use encoding::all::{UTF_8, WINDOWS_1252, ISO_8859_1};
use scraper::{Html, Selector};

#[derive(Debug)]
struct HtmlDocument {
    fragment: Html,
}

impl HtmlDocument { 
    pub fn new(html: String) -> Self {
        let fragment = Html::parse_fragment(&html);
        HtmlDocument { fragment }
    }

}



// 根据 MIME 类型处理数据
pub fn convert_to_readable_text(data: &[u8], mime_type: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match mime_type {
        // HTML/XHTML 内容
        "application/xhtml+xml" | "text/html" => {
            html_to_text(data)
        }
        // CSS 文件
        "text/css" => {
            decode_text(data, "CSS")
        }
        // 纯文本
        "text/plain" => {
            decode_text(data, "Plain text")
        }
        // XML 文件（OPF、NCX等）
        "application/xml" | "text/xml" => {
            decode_text(data, "XML")
        }
        _ => {
            decode_text(data, "Unknown")
        }
    }
}


fn html_to_text(html_data: &[u8]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // 首先解码字节为字符串
    let html_string = attempt_text_decoding(html_data)?;
    // 使用简单的 HTML 到文本转换
    let text = html_to_text_simple(&html_string);
    Ok(text)
}

fn html_to_text_simple(html: &str) -> Vec<String> {
    let mut doc = vec![];
    let document = HtmlDocument::new(html.to_owned());
    let selector = Selector::parse("p").unwrap();
    for element in document.fragment.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join("");
        doc.push(text.trim().to_string());
    }
    doc
}

// 尝试多种编码解码文本
fn attempt_text_decoding(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // 尝试 UTF-8 首先
    if let Ok(text) = String::from_utf8(data.to_vec()) {
        return Ok(text);
    }
    
    // 尝试常见编码
    let encodings: Vec<&dyn Encoding> = vec![UTF_8, WINDOWS_1252, ISO_8859_1];
    
    for encoding in &encodings {
        if let Ok(text) = encoding.decode(data, DecoderTrap::Replace) {
            return Ok(text);
        }
    }
    
    Err("无法解码文本数据".into())
}

// 通用文本解码
fn decode_text(data: &[u8], content_type: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut doc= vec![];
    println!("{}", content_type);
    let _ = attempt_text_decoding(data).map(|text| {
        doc.push(text);
    });
    Ok(doc)
}
