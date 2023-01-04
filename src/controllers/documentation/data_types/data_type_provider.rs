use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::{ArrayElement, HttpDataType, HttpSimpleType};

pub trait DataTypeProvider {
    fn get_data_type() -> HttpDataType;
}

impl DataTypeProvider for u8 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Byte)
    }
}

impl DataTypeProvider for i8 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Byte)
    }
}

impl DataTypeProvider for u16 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Integer)
    }
}

impl DataTypeProvider for i16 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Integer)
    }
}

impl DataTypeProvider for u32 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Integer)
    }
}

impl DataTypeProvider for i32 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Integer)
    }
}

impl DataTypeProvider for u64 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Long)
    }
}

impl DataTypeProvider for i64 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Long)
    }
}

impl DataTypeProvider for f32 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Double)
    }
}

impl DataTypeProvider for f64 {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::Double)
    }
}

impl DataTypeProvider for String {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::String)
    }
}

impl<'s> DataTypeProvider for &'s str {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::String)
    }
}

impl<'s> DataTypeProvider for DateTimeAsMicroseconds {
    fn get_data_type() -> HttpDataType {
        HttpDataType::SimpleType(HttpSimpleType::DateTime)
    }
}

impl<T: DataTypeProvider> DataTypeProvider for Vec<T> {
    fn get_data_type() -> HttpDataType {
        match T::get_data_type() {
            HttpDataType::SimpleType(tp) => HttpDataType::ArrayOf(ArrayElement::SimpleType(tp)),
            HttpDataType::Object(obj) => HttpDataType::ArrayOf(ArrayElement::Object(obj)),
            _ => panic!("Unsupported data type"),
        }
    }
}

impl<TValue: DataTypeProvider> DataTypeProvider for HashMap<String, TValue> {
    fn get_data_type() -> HttpDataType {
        match TValue::get_data_type() {
            HttpDataType::SimpleType(tp) => {
                HttpDataType::DictionaryOf(ArrayElement::SimpleType(tp))
            }
            HttpDataType::Object(obj) => HttpDataType::DictionaryOf(ArrayElement::Object(obj)),
            _ => panic!("Unsupported data type"),
        }
    }
}
