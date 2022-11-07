use crate::controllers::documentation::data_types::HttpField;

pub struct HttpInputParameter {
    pub field: HttpField,
    pub description: String,
    pub source: HttpParameterInputSource,
}

pub enum HttpParameterInputSource {
    Path,
    Query,
    Header,
    FormData,
    Body,
}

impl HttpParameterInputSource {
    pub fn is_query(&self) -> bool {
        match self {
            HttpParameterInputSource::Query => true,
            _ => false,
        }
    }

    pub fn is_form_data(&self) -> bool {
        match self {
            HttpParameterInputSource::FormData => true,
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
            HttpParameterInputSource::FormData => "formData",
            HttpParameterInputSource::Body => "body",
        }
    }
}
