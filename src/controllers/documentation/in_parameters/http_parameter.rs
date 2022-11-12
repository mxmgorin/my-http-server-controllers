use crate::controllers::documentation::data_types::HttpField;

use super::HttpParameterInputSource;

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

    pub fn is_form_data(&self) -> bool {
        match self.source {
            HttpParameterInputSource::FormData => {
                return true;
            }
            _ => false,
        }
    }
}
