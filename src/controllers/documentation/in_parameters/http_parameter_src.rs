pub enum HttpParameterInputSource {
    Path,
    Query,
    Header,
    Body,
    FormData,
}

impl HttpParameterInputSource {
    pub fn is_query(&self) -> bool {
        match self {
            HttpParameterInputSource::Query => true,
            _ => false,
        }
    }

    pub fn is_body(&self) -> bool {
        match self {
            HttpParameterInputSource::Body => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HttpParameterInputSource::Path => "path",
            HttpParameterInputSource::Query => "query",
            HttpParameterInputSource::Header => "header",
            HttpParameterInputSource::Body => "body",
            HttpParameterInputSource::FormData => "form_data",
        }
    }
}
