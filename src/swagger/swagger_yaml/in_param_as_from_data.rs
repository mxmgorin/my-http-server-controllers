use crate::controllers::documentation::{
    ArrayElement, HttpDataType, HttpEnumStructure, HttpField, HttpObjectStructure, HttpSimpleType,
};

use super::yaml_writer::YamlWriter;

pub fn write(yaml_writer: &mut YamlWriter, field: &HttpField) {
    match &field.data_type {
        HttpDataType::SimpleType(simple_type) => {
            yaml_writer.write_empty(field.name.as_str());
            write_simple_type(yaml_writer, simple_type);
        }
        HttpDataType::Object(object) => {
            yaml_writer.write_empty(field.name.as_str());
            write_body_object_type(yaml_writer, object);
        }
        HttpDataType::ArrayOf(array_el) => {
            yaml_writer.write_empty(field.name.as_str());
            write_body_array_type(yaml_writer, array_el);
        }
        HttpDataType::DictionaryOf(array_el) => match array_el {
            crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
                yaml_writer.write_empty(field.name.as_str());
                yaml_writer.increase_level();
                yaml_writer.write("type", "object");
                yaml_writer.write_empty("additionalProperties");
                yaml_writer.increase_level();
                write_simple_type(yaml_writer, simple_type);
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
            }
            crate::controllers::documentation::ArrayElement::Object(obj) => {
                yaml_writer.write_empty(field.name.as_str());
                yaml_writer.increase_level();
                yaml_writer.write("type", "object");
                yaml_writer.write_empty("additionalProperties");
                yaml_writer.increase_level();
                yaml_writer.write_empty("items");
                yaml_writer.increase_level();
                write_body_object_type(yaml_writer, obj);
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
            }

            crate::controllers::documentation::ArrayElement::Enum(enum_type) => {
                panic!(
                    "Enum in dictionary of enum is not supported: {:?}",
                    enum_type
                );
            }
        },
        HttpDataType::DictionaryOfArray(array_el) => {
            yaml_writer.write_empty(field.name.as_str());
            yaml_writer.increase_level();
            yaml_writer.write("type", "object");
            yaml_writer.write_empty("additionalProperties");
            yaml_writer.increase_level();
            write_body_array_type(yaml_writer, array_el);
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        HttpDataType::Enum(enum_data) => {
            yaml_writer.write_empty(field.name.as_str());
            write_enum(yaml_writer, enum_data);
        }
        HttpDataType::None => {}
    }
}

fn write_simple_type(yaml_writer: &mut YamlWriter, simple_type: &HttpSimpleType) {
    yaml_writer.increase_level();

    yaml_writer.write("type", simple_type.as_swagger_type());
    yaml_writer.write("format", simple_type.as_format());

    yaml_writer.decrease_level();
}

fn write_body_object_type(yaml_writer: &mut YamlWriter, object: &HttpObjectStructure) {
    yaml_writer.increase_level();
    yaml_writer.write("type", "object");
    yaml_writer.write_empty("properties");
    yaml_writer.increase_level();
    for obj_field in &object.fields {
        write(yaml_writer, obj_field);
    }

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn write_enum(yaml_writer: &mut YamlWriter, enum_data: &HttpEnumStructure) {
    yaml_writer.increase_level();
    match &enum_data.enum_type {
        crate::controllers::documentation::EnumType::Integer => {
            yaml_writer.write("type", "integer");
        }
        crate::controllers::documentation::EnumType::String => {
            yaml_writer.write("type", "string");
        }
    }

    yaml_writer.decrease_level();
}

fn write_body_array_type(yaml_writer: &mut YamlWriter, array_el: &ArrayElement) {
    match array_el {
        crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
            yaml_writer.increase_level();
            yaml_writer.write("type", "array");
            yaml_writer.write_empty("items");
            write_simple_type(yaml_writer, simple_type);
            yaml_writer.decrease_level();
        }
        crate::controllers::documentation::ArrayElement::Object(obj) => {
            yaml_writer.increase_level();

            yaml_writer.write("type", "array");
            yaml_writer.write_empty("items");
            write_body_object_type(yaml_writer, obj);
            yaml_writer.decrease_level();
        }

        crate::controllers::documentation::ArrayElement::Enum(enum_data) => {
            panic!(
                "Enum in array not supported yet as a from data. {:?}",
                enum_data
            );
        }
    }
}
