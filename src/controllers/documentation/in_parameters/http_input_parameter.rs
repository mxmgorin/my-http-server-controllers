use crate::controllers::documentation::data_types::HttpField;

pub struct HttpInputParameter {
    pub field: HttpField,
    pub description: String,
    pub source: HttpParameterInputSource,
}

impl HttpInputParameter {
    pub fn is_body_reader(&self) -> bool {
        match self.source {
            HttpParameterInputSource::Body => {
                return self.field.data_type.is_simple_type();
            }
            _ => false,
        }
    }
}

pub enum HttpParameterInputSource {
    Path,
    Query,
    Header,
    Body,
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
}

impl HttpParameterInputSource {
    pub fn as_str(&self) -> &str {
        match self {
            HttpParameterInputSource::Path => "path",
            HttpParameterInputSource::Query => "query",
            HttpParameterInputSource::Header => "header",
            HttpParameterInputSource::Body => "body",
        }
    }
}
