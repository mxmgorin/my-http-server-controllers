use std::str::FromStr;

use my_http_server::HttpFailResult;

pub struct FileContent {
    pub file_name: String,
    pub content: Vec<u8>,
}

impl FromStr for FileContent {
    type Err = HttpFailResult;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = FileContent {
            file_name: s.to_string(),
            content: Vec::new(),
        };

        Ok(result)
    }
}
