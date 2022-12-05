use super::HttpDataType;
#[derive(Clone)]
pub struct HttpEnumCase {
    pub id: i16,
    pub value: String,
    pub description: String,
}
#[derive(Clone)]
pub enum EnumType {
    Integer,
    String,
}
#[derive(Clone)]
pub struct HttpEnumStructure {
    pub struct_id: String,
    pub enum_type: EnumType,
    pub cases: Vec<HttpEnumCase>,
}

impl HttpEnumStructure {
    pub fn into_http_data_type_object(self) -> HttpDataType {
        HttpDataType::Enum(self)
    }
}
