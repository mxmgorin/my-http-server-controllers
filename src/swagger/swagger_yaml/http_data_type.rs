use crate::controllers::documentation::data_types::{ArrayElement, HttpDataType, HttpSimpleType};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, root_name: &str, data_type: &HttpDataType) {
    match &data_type {
        HttpDataType::SimpleType(param_type) => {
            yaml_writer.write_empty(root_name);
            write_simple_type(yaml_writer, param_type);
        }

        HttpDataType::Object(object_type) => {
            yaml_writer.write_empty(root_name);
            write_object_type(yaml_writer, &object_type.struct_id);
        }
        HttpDataType::Enum(enum_type) => match enum_type.enum_type {
            crate::controllers::documentation::data_types::EnumType::Integer => {
                yaml_writer.write_empty(root_name);
                write_simple_type(yaml_writer, &HttpSimpleType::Integer);
            }
            crate::controllers::documentation::data_types::EnumType::String => {
                yaml_writer.write_empty(root_name);
                write_object_type(yaml_writer, &enum_type.struct_id);
            }
        },
        HttpDataType::None => {}
        HttpDataType::ArrayOf(array_element) => {
            yaml_writer.write_empty(root_name);
            yaml_writer.increase_level();
            yaml_writer.write("type", "array");

            yaml_writer.write_empty("items");

            match array_element {
                ArrayElement::SimpleType(param_type) => write_simple_type(yaml_writer, param_type),
                ArrayElement::Object(object_type) => {
                    write_object_type(yaml_writer, &object_type.struct_id)
                }
            };

            yaml_writer.decrease_level();
        }
        HttpDataType::DictionaryOf(array_element) => {
            yaml_writer.write_empty(root_name);
            yaml_writer.increase_level();
            yaml_writer.write("type", "object");

            yaml_writer.write_empty("additionalProperties");

            match array_element {
                ArrayElement::SimpleType(param_type) => write_simple_type(yaml_writer, param_type),
                ArrayElement::Object(object_type) => {
                    write_object_type(yaml_writer, &object_type.struct_id)
                }
            };

            yaml_writer.decrease_level();
        }
    }
}

fn write_simple_type(yaml_writer: &mut YamlWriter, param_type: &HttpSimpleType) {
    yaml_writer.increase_level();
    yaml_writer.write("type", param_type.as_swagger_type());
    yaml_writer.write("format", param_type.as_format());

    yaml_writer.decrease_level();
}

fn write_object_type(yaml_writer: &mut YamlWriter, struct_id: &str) {
    yaml_writer.increase_level();
    yaml_writer.write(
        "$ref",
        format!("'#/components/schemas/{}'", struct_id).as_str(),
    );
    yaml_writer.decrease_level();
}
