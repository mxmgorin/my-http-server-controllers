use super::{HttpObjectStructure, HttpSimpleType};
#[derive(Clone)]
pub enum ArrayElement {
    SimpleType(HttpSimpleType),
    ObjectId { struct_id: &'static str },
    Object(HttpObjectStructure),
}
