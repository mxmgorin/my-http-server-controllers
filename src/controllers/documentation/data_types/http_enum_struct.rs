use super::HttpDataType;
#[derive(Clone, Debug)]
pub struct HttpEnumCase {
    pub id: i16,
    pub value: &'static str,
    pub description: &'static str,
}
#[derive(Clone, Debug)]
pub enum EnumType {
    Integer,
    String,
}
#[derive(Clone, Debug)]
pub struct HttpEnumStructure {
    pub struct_id: &'static str,
    pub enum_type: EnumType,
    pub cases: Vec<HttpEnumCase>,
}

impl HttpEnumStructure {
    pub fn into_http_data_type_object(self) -> HttpDataType {
        HttpDataType::Enum(self)
    }
}

impl Into<HttpDataType> for HttpEnumStructure {
    fn into(self) -> HttpDataType {
        self.into_http_data_type_object()
    }
}
