use crate::{
    controllers::documentation::{in_parameters::HttpInputParameter, HttpDataType},
    swagger::swagger_yaml::yaml_writer::YamlWriter,
};

use super::write_enum_type;

pub fn write_query_input_param(yaml_writer: &mut YamlWriter, input_param: &HttpInputParameter) {
    match &input_param.field.data_type {
        HttpDataType::SimpleType(simple_type) => {
            yaml_writer.increase_level();
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.write("description", input_param.description.as_str());
            super::write_simple_type(yaml_writer, simple_type, input_param.field.required);
            yaml_writer.decrease_level();
        }
        HttpDataType::Object(_) => {
            panic!("Object type is not supported for non body parameter")
        }
        HttpDataType::ArrayOf(array_el) => match array_el {
            crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
                yaml_writer.increase_level();
                yaml_writer.write(
                    "name",
                    format!("{}[]", input_param.field.name.as_str()).as_str(),
                );
                yaml_writer.write("description", input_param.description.as_str());
                super::write_array_input_param(yaml_writer, simple_type);
                yaml_writer.decrease_level();
            }
            crate::controllers::documentation::ArrayElement::Object(_) => {
                panic!("Array of object type is not supported for non body parameter")
            }

            crate::controllers::documentation::ArrayElement::Enum(enum_data) => {
                yaml_writer.increase_level();
                yaml_writer.write(
                    "name",
                    format!("{}[]", input_param.field.name.as_str()).as_str(),
                );
                yaml_writer.write("description", input_param.description.as_str());
                super::write_array_enum_case(yaml_writer, enum_data);
                yaml_writer.decrease_level();
            }
        },
        HttpDataType::DictionaryOf(_) => {
            panic!("Dictionary can not be used as a non body parameter")
        }
        HttpDataType::DictionaryOfArray(_) => {
            panic!("Dictionary of array can not be used as a non body parameter")
        }
        HttpDataType::Enum(enum_data) => {
            yaml_writer.increase_level();
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.write("description", input_param.description.as_str());
            write_enum_type(yaml_writer, enum_data);
            yaml_writer.decrease_level();
        }
        HttpDataType::None => {
            panic!("Somehow we have non parameter")
        }
    }
}
