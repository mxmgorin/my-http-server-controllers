use my_http_server::{HttpFailResult, HttpRequest};

pub struct FileContent {
    pub file_name: String,
    pub content: Vec<u8>,
}

impl FileContent {
    pub async fn read_from_body(request: &mut HttpRequest) -> Result<Self, HttpFailResult> {
        let headers = request.get_headers();
        for header in headers {
            println!("{:?}", header);
        }
        let http_body = request.receive_body().await?;
        let result = Self {
            file_name: "test".to_string(),
            content: http_body.get_body(),
        };
        Ok(result)
    }
}
