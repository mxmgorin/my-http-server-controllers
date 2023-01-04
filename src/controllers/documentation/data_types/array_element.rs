use super::{HttpObjectStructure, HttpSimpleType};
#[derive(Clone)]
pub enum ArrayElement {
    SimpleType(HttpSimpleType),
    Object(HttpObjectStructure),
}
