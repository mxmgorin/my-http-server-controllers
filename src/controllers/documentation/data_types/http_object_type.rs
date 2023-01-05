use super::{ArrayElement, HttpDataType, HttpField};

#[derive(Clone, Debug)]
pub struct HttpObjectStructure {
    pub struct_id: &'static str,
    pub fields: Vec<HttpField>,
}

impl HttpObjectStructure {
    pub fn into_http_data_type_object(self) -> HttpDataType {
        HttpDataType::Object(self)
    }

    pub fn into_http_data_type_array(self) -> HttpDataType {
        HttpDataType::ArrayOf(ArrayElement::Object(self))
    }

    pub fn new(struct_id: &'static str) -> Self {
        Self {
            struct_id,
            fields: vec![],
        }
    }
}
