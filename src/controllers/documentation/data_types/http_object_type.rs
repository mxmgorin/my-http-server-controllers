use rust_extensions::StrOrString;

use super::{ArrayElement, HttpDataType, HttpField};

#[derive(Clone, Debug)]
pub struct HttpObjectStructure {
    pub struct_id: &'static str,

    pub generic_struct_id: Option<&'static str>,
    pub fields: Vec<HttpField>,
}

impl super::InputStructure for HttpObjectStructure {
    fn get_struct_id(&self) -> StrOrString<'static> {
        match self.generic_struct_id {
            Some(generic_struct_id) => format!("{}_{}", self.struct_id, generic_struct_id).into(),
            None => self.struct_id.into(),
        }
    }
}

impl HttpObjectStructure {
    pub fn into_http_data_type_object(self) -> HttpDataType {
        HttpDataType::Object(self)
    }

    pub fn into_http_data_type_array(self) -> HttpDataType {
        HttpDataType::ArrayOf(ArrayElement::Object(self))
    }

    pub fn new(struct_id: &'static str, generic_struct_id: Option<&'static str>) -> Self {
        Self {
            struct_id,
            generic_struct_id,
            fields: vec![],
        }
    }
}
